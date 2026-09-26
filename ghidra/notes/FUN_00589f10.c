
bool __thiscall FUN_00589f10(void *this,undefined4 param_1)

{
  void *this_00;
  int iVar1;
  uint *puVar2;
  bool bVar3;
  int aiStack_1c [2];
  int aiStack_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0064fbc8;
  pvStack_c = ExceptionList;
  bVar3 = true;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(**(int **)((int)this + 0x14) + 0x1c8))();
  if ((iVar1 != 0) && (*(int *)((int)this + 0x4c) == 0)) {
    this_00 = (void *)((int)this + 8);
    FUN_00589080(aiStack_14,this_00);
    iStack_4 = 0;
    iVar1 = FUN_00587b90(this_00,aiStack_14,param_1);
    bVar3 = iVar1 != 0;
    puVar2 = FUN_0048a640(this_00,aiStack_1c);
    iStack_4._0_1_ = 1;
    iVar1 = FUN_004ece60(puVar2);
    iStack_4._0_1_ = 0;
    FUN_00619730();
    if (iVar1 != 0) {
      *(undefined4 *)((int)this + 0x4c) = 1;
      FUN_00589440(aiStack_1c,this_00);
      iStack_4._0_1_ = 2;
      iVar1 = FUN_00587f60(this_00,aiStack_1c,param_1);
      if ((iVar1 == 0) || (!bVar3)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      FUN_00589480(aiStack_1c);
    }
    iStack_4 = 0xffffffff;
    FUN_005890c0(aiStack_14);
  }
  ExceptionList = pvStack_c;
  return bVar3;
}

