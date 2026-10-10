#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0
"""Apply documented, response-only enum references; never rewrite request schemas.

Values are product-specific facts. Sources are explicit per component; unspecified
fields retain their current native representation. Rate readers preserve behavior.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPOT = 'https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md'
INTERVALS = ['1m','3m','5m','15m','30m','1h','2h','4h','6h','8h','12h','1d','3d','1w','1M']
INTERVAL_NAMES = {v: {'s':'Second','m':'Minute','h':'Hour','d':'Day','w':'Week','M':'Month'}[v[-1]]+v[:-1] for v in ['1s',*INTERVALS]}
ORDER_TYPES = ['LIMIT','MARKET','STOP','STOP_MARKET','TAKE_PROFIT','TAKE_PROFIT_MARKET','TRAILING_STOP_MARKET']
PRICE_MATCH = ['NONE','OPPONENT','OPPONENT_5','OPPONENT_10','OPPONENT_20','QUEUE','QUEUE_5','QUEUE_10','QUEUE_20']


def fact(name, values, sources, variants=None):
    result = {'type':'integer' if isinstance(values[0],int) else 'string', 'enum':values,
              'description':f'Native `{name}` values documented for this product.', 'x-sources':sources}
    if variants: result['x-rust-variants'] = dict(zip(map(str,values),variants)) if isinstance(variants,list) else variants
    return result


def facts(product):
    if product in ['usdm','coinm']:
        family = 'usds' if product == 'usdm' else 'coin'
        common = f'https://developers.binance.com/en/docs/products/derivatives-trading-{family}-futures/common-definition'
        events = f'https://developers.binance.com/en/docs/products/derivatives-trading-{family}-futures/user-data-streams'
        trade = f'https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-{"usd-s-m" if product == "usdm" else "coin-m"}-futures/api/rest-api/trade#new-order'
        definitions = {
            'OrderSide':(['BUY','SELL'],[common]),
            'OrderStatus':(['NEW','PARTIALLY_FILLED','FILLED','CANCELED',*(['REJECTED'] if product=='usdm' else []),'EXPIRED','EXPIRED_IN_MATCH'],[common,events]),
            'OrderType':([*ORDER_TYPES,'LIQUIDATION'],[common,events]),
            'PositionSide':(['BOTH','LONG','SHORT'],[common]),
            'TimeInForce':(['GTC','IOC','FOK','GTX',*(['GTD','RPI'] if product=='usdm' else [])],[common]),
            'WorkingType':(['MARK_PRICE','CONTRACT_PRICE'],[common]),
            'SelfTradePreventionMode':(['NONE','EXPIRE_TAKER','EXPIRE_BOTH','EXPIRE_MAKER'],[common,trade]),
            'PriceMatch':(PRICE_MATCH,[common]),
            'FilterType':(['PRICE_FILTER','LOT_SIZE','MARKET_LOT_SIZE','MAX_NUM_ORDERS',*(['MAX_NUM_ALGO_ORDERS'] if product=='usdm' else []),'PERCENT_PRICE',*(['MIN_NOTIONAL'] if product=='usdm' else [])],[common]),
            'ExecutionType':(['NEW','CANCELED','CALCULATED','EXPIRED','TRADE','AMENDMENT'],[events]),
            'AccountUpdateReason':(['DEPOSIT','WITHDRAW','ORDER','FUNDING_FEE',*(['WITHDRAW_REJECT'] if product=='usdm' else []),'ADJUSTMENT','INSURANCE_CLEAR','ADMIN_DEPOSIT','ADMIN_WITHDRAW','MARGIN_TRANSFER','MARGIN_TYPE_CHANGE','ASSET_TRANSFER',*(['OPTIONS_PREMIUM_FEE','OPTIONS_SETTLE_PROFIT','AUTO_EXCHANGE'] if product=='usdm' else []),'COIN_SWAP_DEPOSIT','COIN_SWAP_WITHDRAW'],[events]),
            'RateLimitType':(['REQUEST_WEIGHT','ORDERS'],[common]),
            'RateLimitInterval':((['SECOND'] if product=='usdm' else [])+['MINUTE'],[common,*([f'https://developers.binance.com/en/docs/products/derivatives-trading-{family}-futures/websocket-api-general-info'] if product=='usdm' else [])]),
        }
        result = {name:fact(name,*values) for name,values in definitions.items()}
        algo_source='https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/user-data-streams'
        algo_sources=[algo_source] if product=='usdm' else ['https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice',algo_source]
        result['AlgoStatus']=fact('AlgoStatus',['NEW','CANCELED','TRIGGERING','TRIGGERED','FINISHED','REJECTED','EXPIRED'],algo_sources)
        result['AlgoType']=fact('AlgoType',['CONDITIONAL'],[trade.replace('#new-order','#new-algo-order')] if product=='usdm' else [algo_sources[0],'https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-algo-order'])
        result['KlineInterval'] = fact('KlineInterval',(['1s'] if product=='usdm' else [])+INTERVALS,[common],INTERVAL_NAMES)
        return result
    if product == 'spot':
        definitions = {
            'OrderSide':['BUY','SELL'],
            'OrderStatus':['NEW','PENDING_NEW','PARTIALLY_FILLED','FILLED','CANCELED','PENDING_CANCEL','REJECTED','EXPIRED','EXPIRED_IN_MATCH'],
            'OrderType':['LIMIT','MARKET','STOP_LOSS','STOP_LOSS_LIMIT','TAKE_PROFIT','TAKE_PROFIT_LIMIT','LIMIT_MAKER'],
            'TimeInForce':['GTC','IOC','FOK'],
            'SelfTradePreventionMode':['NONE','EXPIRE_MAKER','EXPIRE_TAKER','EXPIRE_BOTH','DECREMENT','TRANSFER'],
            'ListStatusType':['RESPONSE','EXEC_STARTED','UPDATED','ALL_DONE'],
            'ListOrderStatus':['EXECUTING','ALL_DONE','REJECT'],
            'ContingencyType':['OCO','OTO'],
            'ExecutionType':['NEW','CANCELED','REPLACED','REJECTED','TRADE','EXPIRED','TRADE_PREVENTION'],
            'Permission':['SPOT','MARGIN','LEVERAGED',*[f'TRD_GRP_{i:03}' for i in range(2,26)]],
            'AllocationType':['SOR'], 'WorkingFloor':['EXCHANGE','SOR'],
        }
        result = {name:fact(name,values,[SPOT]) for name,values in definitions.items()}
        sbe='https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/sbe/schemas/spot_3_4.xml'
        result['RateLimitType']=fact('RateLimitType',['REQUEST_WEIGHT','ORDERS','RAW_REQUESTS','CONNECTIONS'],[SPOT,sbe])
        result['RateLimitInterval']=fact('RateLimitInterval',['SECOND','MINUTE','HOUR','DAY'],[SPOT,sbe])
        result['KlineInterval'] = fact('KlineInterval',['1s',*INTERVALS],['https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/web-socket-streams.md#klinecandlestick-streams'],INTERVAL_NAMES)
        return result
    if product == 'wallet':
        deposit='https://developers.binance.com/legacy-docs/wallet/capital/deposite-history'
        withdraw='https://developers.binance.com/legacy-docs/wallet/capital/withdraw-history'
        return {
            'DepositStatus':fact('DepositStatus',[0,1,2,6,7,8],[deposit],['Pending','Success','Rejected','CreditedNotWithdrawable','WrongDeposit','WaitingUserConfirmation']),
            'WithdrawStatus':fact('WithdrawStatus',[0,2,3,4,6],[withdraw],['EmailSent','AwaitingApproval','Rejected','Processing','Completed']),
            'TransferDirection':fact('TransferDirection',[0,1],[deposit,withdraw],['External','Internal']),
            'WalletType':fact('WalletType',[0,1],[withdraw],['Spot','Funding']),
            'DepositTravelRuleStatus':fact('DepositTravelRuleStatus',[0,1],[deposit],['Ready','InformationRequired']),
            'SystemStatus':fact('SystemStatus',[0,1],['https://developers.binance.com/legacy-docs/wallet/others/system-status'],['Normal','Maintenance']),
            'TravelRuleStatus':fact('TravelRuleStatus',[0,1,2],['https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule','https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v1'],['Completed','Pending','Failed']),
            'TravelRuleVerificationStatus':fact('TravelRuleVerificationStatus',['PASSED','PENDING','REJECTED'],['https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule']),
            'CloudMiningPaymentType':fact('CloudMiningPaymentType',[248,249],['https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history'],['Payment','Refund']),
            'UniversalTransferType':fact('UniversalTransferType',['MAIN_UMFUTURE','MAIN_CMFUTURE','MAIN_MARGIN','UMFUTURE_MAIN','UMFUTURE_MARGIN','CMFUTURE_MAIN','CMFUTURE_MARGIN','MARGIN_MAIN','MARGIN_UMFUTURE','MARGIN_CMFUTURE','ISOLATEDMARGIN_MARGIN','MARGIN_ISOLATEDMARGIN','ISOLATEDMARGIN_ISOLATEDMARGIN','MAIN_FUNDING','FUNDING_MAIN','FUNDING_UMFUTURE','UMFUTURE_FUNDING','MARGIN_FUNDING','FUNDING_MARGIN','FUNDING_CMFUTURE','CMFUTURE_FUNDING','MAIN_OPTION','OPTION_MAIN','UMFUTURE_OPTION','OPTION_UMFUTURE','MARGIN_OPTION','OPTION_MARGIN','FUNDING_OPTION','OPTION_FUNDING','MAIN_PORTFOLIO_MARGIN','PORTFOLIO_MARGIN_MAIN'],['https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-universal-transfer']),
            'CloudMiningStatus':fact('CloudMiningStatus',['S'],['https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history']),
        }
    if product == 'convert':
        return {'OrderStatus':fact('OrderStatus',['PROCESS','ACCEPT_SUCCESS','SUCCESS','FAIL'],['https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#accept-quote'])}
    return {}


def reference(name, original):
    # Retain field-level nullable/decimal/parser evidence; remove inline enum/type.
    return {k:v for k,v in original.items() if k not in ['type','enum','format','items']} | {'$ref':f'#/components/schemas/{name}'}


def apply(product, kind):
    path=ROOT/'schema'/f'{product}-{kind}.json'
    if not path.exists(): return
    data=json.loads(path.read_text())
    components=data['components'].setdefault('schemas',{})
    definitions=facts(product)
    used=set()
    seen=set()

    def set_field(properties,key,name):
        value=properties.get(key)
        if not value or name not in definitions: return
        if value.get('type') not in [None,'string','integer']: return
        properties[key]=reference(name,value); used.add(name)

    def set_array(properties,key,name,levels=1):
        value=properties.get(key)
        if not value: return
        for _ in range(levels):
            if value.get('type') != 'array':return
            parent=value; value=value.get('items',{})
        parent['items']=reference(name,value); used.add(name)

    def walk(node,operation='',context=''):
        if not isinstance(node,dict):return
        if '$ref' in node:
            name=node['$ref'].split('/')[-1]
            if name in seen:return
            seen.add(name)
            if name in components and not components[name].get('enum'):walk(components[name],operation,name)
            return
        props=node.get('properties',{})
        if product in ['usdm','coinm','spot']:
            for key,name in {'rateLimitType':'RateLimitType','interval':'RateLimitInterval','side':'OrderSide','orderType':'OrderType','actualType':'OrderType','tpOrderType':'OrderType','algoStatus':'AlgoStatus','algoType':'AlgoType','positionSide':'PositionSide','timeInForce':'TimeInForce','workingType':'WorkingType','priceMatch':'PriceMatch','selfTradePreventionMode':'SelfTradePreventionMode','defaultSelfTradePreventionMode':'SelfTradePreventionMode','contingencyType':'ContingencyType','listStatusType':'ListStatusType','listOrderStatus':'ListOrderStatus','allocationType':'AllocationType','workingFloor':'WorkingFloor'}.items():
                if key!='interval' or {'intervalNum','limit'} <= props.keys():set_field(props,key,name)
            if product in ['usdm','coinm'] and context=='accountUpdate' and 'm' in props and ({'B','P'} & props.keys()):set_field(props,'m','AccountUpdateReason')
            order = bool({'orderId','clientOrderId','origType','executedQty','algoId'} & props.keys()) or {'S','o','f'} <= props.keys()
            if order:
                for key,name in {'status':'OrderStatus','type':'OrderType','origType':'OrderType','S':'OrderSide','o':'OrderType','ot':'OrderType','f':'TimeInForce','x':'ExecutionType','X':'OrderStatus','wt':'WorkingType','ps':'PositionSide','V':'SelfTradePreventionMode','pm':'PriceMatch'}.items():set_field(props,key,name)
            if product=='usdm' and context=='tradeLite':set_field(props,'S','OrderSide')
            if product=='usdm' and context=='algoUpdate':
                for key,name in {'at':'AlgoType','X':'AlgoStatus','act':'OrderType'}.items():set_field(props,key,name)
            if product=='spot' and context=='executionReport':
                for key,name in {'S':'OrderSide','o':'OrderType','f':'TimeInForce','x':'ExecutionType','X':'OrderStatus','V':'SelfTradePreventionMode','b':'AllocationType','k':'WorkingFloor'}.items():set_field(props,key,name)
            if product=='spot' and context=='listStatus':
                for key,name in {'c':'ContingencyType','l':'ListStatusType','L':'ListOrderStatus'}.items():set_field(props,key,name)
            if 'ps' in props and 'pa' in props:set_field(props,'ps','PositionSide')
            if {'i','o','c','h','l'} <= props.keys():set_field(props,'i','KlineInterval')
            if product != 'spot':set_field(props,'filterType','FilterType')
            for key,name in {'orderTypes':'OrderType','timeInForce':'TimeInForce','permissions':'Permission','allowedSelfTradePreventionModes':'SelfTradePreventionMode'}.items():
                if name in definitions:set_array(props,key,name)
            if 'Permission' in definitions:set_array(props,'permissionSets','Permission',2)
        elif product=='convert':
            set_field(props,'orderStatus','OrderStatus')
        elif product=='wallet':
            names={
                'depositHistory':{'status':'DepositStatus','transferType':'TransferDirection','walletType':'WalletType','travelRuleStatus':'DepositTravelRuleStatus'},
                'withdrawHistory':{'status':'WithdrawStatus','transferType':'TransferDirection','walletType':'WalletType'},
                'systemStatus':{'status':'SystemStatus'},
                'queryUserUniversalTransferHistory':{'type':'UniversalTransferType'},
                'depositHistoryTravelRule':{'travelRuleStatus':'TravelRuleStatus','travelRuleStatusV2':'TravelRuleVerificationStatus'},
                'withdrawHistoryV1':{'travelRuleStatus':'TravelRuleStatus'},
                'withdrawHistoryV2':{'travelRuleStatus':'TravelRuleStatus'},
                'getCloudMiningPaymentAndRefundHistory':{'type':'CloudMiningPaymentType','status':'CloudMiningStatus'},
            }.get(operation,{})
            for key,name in names.items():set_field(props,key,name)
        for key,value in list(node.items()):
            if key in ['properties']:
                for name,field in list(value.items()):walk(field,operation,name if name in ['executionReport','listStatus'] else context)
            elif isinstance(value,dict):walk(value,operation,context)
            elif isinstance(value,list):
                for child in value:walk(child,operation,context)

    for operation in data['operations']:walk(operation.get('responses',{}),operation.get('operationId',''))
    for name in list(components):
        if not components[name].get('enum'):walk(components[name],context=name)
    # The COIN-M native ACCOUNT_UPDATE position break-even price is financial.
    if product=='coinm' and kind=='streams':
        components['accountUpdate']['properties']['a']['properties']['P']['items']['properties']['bep']['x-decimal']=True
    for name in used:components[name]=definitions[name]
    path.write_text(json.dumps(data,indent=2,ensure_ascii=False)+'\n')
    print(f'{product}-{kind}: '+', '.join(sorted(used)))


if __name__=='__main__':
    for product in ['usdm','coinm','spot','wallet','convert']:
        for kind in ['rest','ws','streams']:apply(product,kind)
