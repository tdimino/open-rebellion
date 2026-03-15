
undefined4 __cdecl FUN_0040a700(undefined4 param_1,void *param_2)

{
  void *pvVar1;
  undefined4 uVar2;
  undefined4 local_e4 [3];
  undefined4 local_d8 [3];
  undefined4 local_cc [3];
  undefined4 local_c0 [3];
  undefined4 local_b4 [3];
  undefined4 local_a8 [3];
  undefined4 local_9c [3];
  undefined4 local_90 [3];
  undefined4 local_84 [3];
  undefined4 local_78 [3];
  undefined4 local_6c [3];
  undefined4 local_60 [3];
  undefined4 local_54 [3];
  undefined4 local_48 [3];
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629e5b;
  local_c = ExceptionList;
  uVar2 = 1;
  switch(param_1) {
  case 0:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_24,&DAT_006a80dc);
    local_4 = 0;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_24);
    break;
  case 1:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_cc,s_STRAT_INIT_006a80d0);
    local_4 = 1;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_cc);
    break;
  case 2:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_6c,s_STRAT_006a80c8);
    local_4 = 2;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_6c);
    break;
  case 3:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_b4,s_STRAT_ALL_BATTLE_START_006a80b0);
    local_4 = 3;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_b4);
    break;
  case 4:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_3c,s_STRAT_BATTLE_START_006a809c);
    local_4 = 4;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_3c);
    break;
  case 5:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_9c,s_STRAT_BATTLE_CHOOSE_TYPE_006a8080);
    local_4 = 5;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_9c);
    break;
  case 6:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_54,s_STRAT_BATTLE_WAIT_FOR_TYPE_CHOIC_006a805c);
    local_4 = 6;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_54);
    break;
  case 7:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_84,s_STRAT_BATTLE_CONVERT_TO_TACT_006a803c);
    local_4 = 7;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_84);
    break;
  case 8:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_e4,s_STRAT_BATTLE_CONVERT_TO_TACT_ABS_006a8014);
    local_4 = 8;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_e4);
    break;
  case 9:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_d8,s_STRAT_BATTLE_FINISH_006a8000);
    local_4 = 9;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_d8);
    break;
  case 10:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_c0,s_STRAT_BATTLE_CHOOSE_DONE_006a7fe4);
    local_4 = 10;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_c0);
    break;
  case 0xb:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_a8,s_STRAT_BATTLE_WAIT_FOR_DONE_CHOIC_006a7fc0);
    local_4 = 0xb;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_a8);
    break;
  case 0xc:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_90,s_STRAT_ALL_BATTLE_FINISH_006a7fa8);
    local_4 = 0xc;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_90);
    break;
  default:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_18,s_Invalid__006a7f4c);
    local_4 = 0x11;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
    uVar2 = 0;
    break;
  case 0x14:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_78,&DAT_006a7fa0);
    local_4 = 0xd;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_78);
    break;
  case 0x15:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_48,s_TACT_ABSTRACT_006a7f78);
    local_4 = 0xf;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_48);
    break;
  case 0x16:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_60,s_TACT_CONVERT_TO_STRAT_006a7f88);
    local_4 = 0xe;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_60);
    break;
  case 0x17:
    ExceptionList = &local_c;
    pvVar1 = FUN_005f35b0(local_30,s_TACT_ABSTRACT_CONVERT_TO_STRAT_006a7f58);
    local_4 = 0x10;
    FUN_005f3090(param_2,(int)pvVar1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_30);
  }
  ExceptionList = local_c;
  return uVar2;
}

