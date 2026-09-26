
int __thiscall FUN_0051fc00(void *this,int *param_1)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 auStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_006433b0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_1 + 4))();
  FUN_00541e70(auStack_1c);
  iStack_4 = 0;
  FUN_0051fcb0(this,auStack_1c);
  uStack_20 = 0xffffffff;
  uStack_24 = 1;
  puVar2 = &uStack_24;
  iStack_4._0_1_ = 1;
  iVar1 = (**(code **)(*(int *)this + 0x28))();
  iVar1 = FUN_0053f150(auStack_1c,iVar1,param_1,puVar2);
  iStack_4 = (uint)iStack_4._1_3_ << 8;
  FUN_00619730();
  iStack_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return iVar1;
}

