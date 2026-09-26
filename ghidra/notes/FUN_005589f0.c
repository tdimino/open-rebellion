
undefined4 __thiscall FUN_005589f0(void *this,void *param_1)

{
  uint uVar1;
  char *pcVar2;
  undefined4 local_54 [3];
  undefined4 local_48 [3];
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649910;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f35b0(local_18,&DAT_006aa5d0);
  local_4 = 0;
  uVar1 = *(uint *)this >> 4 & 3;
  if (uVar1 == 1) {
    pcVar2 = s_Alliance_006a8f04;
  }
  else if (uVar1 == 2) {
    pcVar2 = s_Empire_006a8f10;
  }
  else if (uVar1 == 3) {
    pcVar2 = s_Neutral_006a8f1c;
  }
  else {
    pcVar2 = s_UNKNOWN__006a8f28;
  }
  FUN_005f35b0(local_24,pcVar2);
  local_4._0_1_ = 1;
  FUN_005f35b0(local_30,&DAT_006ab0b0);
  local_4._0_1_ = 2;
  uVar1 = *(uint *)this >> 2 & 3;
  if (uVar1 == 1) {
    pcVar2 = s_Alliance_006a8f04;
  }
  else if (uVar1 == 2) {
    pcVar2 = s_Empire_006a8f10;
  }
  else if (uVar1 == 3) {
    pcVar2 = s_Neutral_006a8f1c;
  }
  else {
    pcVar2 = s_UNKNOWN__006a8f28;
  }
  FUN_005f35b0(local_3c,pcVar2);
  local_4._0_1_ = 3;
  FUN_005f35b0(local_48,&DAT_006ab0a8);
  local_4._0_1_ = 4;
  uVar1 = *(uint *)this & 3;
  if (uVar1 == 1) {
    pcVar2 = s_Alliance_006a8f04;
  }
  else if (uVar1 == 2) {
    pcVar2 = s_Empire_006a8f10;
  }
  else if (uVar1 == 3) {
    pcVar2 = s_Neutral_006a8f1c;
  }
  else {
    pcVar2 = s_UNKNOWN__006a8f28;
  }
  FUN_005f35b0(local_54,pcVar2);
  local_4._0_1_ = 5;
  FUN_005f30d0(param_1,(int)local_54);
  FUN_005f30d0(param_1,(int)local_48);
  FUN_005f30d0(param_1,(int)local_3c);
  FUN_005f30d0(param_1,(int)local_30);
  FUN_005f30d0(param_1,(int)local_24);
  FUN_005f30d0(param_1,(int)local_18);
  local_4._0_1_ = 4;
  FUN_005f2ff0(local_54);
  local_4._0_1_ = 3;
  FUN_005f2ff0(local_48);
  local_4._0_1_ = 2;
  FUN_005f2ff0(local_3c);
  local_4._0_1_ = 1;
  FUN_005f2ff0(local_30);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_24);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return 1;
}

