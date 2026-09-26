
uint __thiscall FUN_004f5380(void *this,int *param_1)

{
  void *this_00;
  int iVar1;
  uint uVar2;
  undefined4 auStack_24 [2];
  int aiStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063f900;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_1 + 4))();
  this_00 = (void *)thunk_FUN_00506e40();
  FUN_00520570(auStack_24);
  uStack_4 = 0;
  FUN_00541e70(aiStack_1c);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_0051fcb0(this,aiStack_1c);
  uVar2 = 0;
  if (this_00 != (void *)0x0) {
    iVar1 = (**(code **)(*(int *)this + 0x28))();
    uVar2 = FUN_005422f0(this_00,aiStack_1c,iVar1,(int)this + 0x58,param_1,auStack_24);
  }
  uStack_4 = uStack_4 & 0xffffff00;
  FUN_00541ea0();
  uStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar2;
}

