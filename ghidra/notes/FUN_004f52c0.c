
bool __thiscall FUN_004f52c0(void *this,int *param_1)

{
  bool bVar1;
  void *this_00;
  int iVar2;
  undefined4 auStack_24 [2];
  int aiStack_1c [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063f8e0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  (**(code **)(*param_1 + 4))();
  this_00 = (void *)thunk_FUN_00506e40();
  FUN_00520570(auStack_24);
  uStack_4 = 0;
  FUN_00541e70(aiStack_1c);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  FUN_0051fcb0(this,aiStack_1c);
  bVar1 = false;
  if (this_00 != (void *)0x0) {
    iVar2 = (**(code **)(*(int *)this + 0x28))();
    bVar1 = FUN_005420d0(this_00,aiStack_1c,iVar2,(int)this + 0x58,param_1,auStack_24);
  }
  uStack_4 = uStack_4 & 0xffffff00;
  FUN_00541ea0();
  uStack_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return bVar1;
}

