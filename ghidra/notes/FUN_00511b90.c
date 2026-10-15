
undefined4 __thiscall FUN_00511b90(int param_1,int param_2,int param_3,undefined4 param_4)

{
  int iVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined1 auStack_18 [12];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00642160;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_0053a010();
  uVar3 = 0;
  if (iVar1 != 0) {
    FUN_005f2f50();
    iStack_4 = 0;
    FUN_005f35b0(s_Delta__006a99f8);
    iStack_4._0_1_ = 1;
    FUN_005f30d0(auStack_18);
    FUN_005f31a0(param_3 - param_2);
    iStack_4 = (uint)iStack_4._1_3_ << 8;
    FUN_005f2ff0();
    uVar3 = param_4;
    uVar2 = FUN_00583c40(param_4);
    FUN_004f8980(s_SystemLoyaltyNotif_006a99dc,s_Loyalty_006a99f0,param_3,param_2,uVar2,uVar3);
    if (((byte)*(undefined4 *)(param_1 + 0x24) & 0x30) == 0x20) {
      param_3 = 100 - param_3;
      param_2 = 100 - param_2;
    }
    uVar3 = FUN_0053fcf0(0x140,param_1,param_2,param_3,param_4);
    iStack_4 = 0xffffffff;
    FUN_005f2ff0();
  }
  ExceptionList = pvStack_c;
  return uVar3;
}

