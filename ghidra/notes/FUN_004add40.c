
undefined4 __thiscall FUN_004add40(void *this,uint param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 uVar3;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638878;
  pvStack_c = ExceptionList;
  uVar3 = 1;
  ExceptionList = &pvStack_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  local_4 = 0;
  piVar1 = (int *)FUN_005f5500((void *)(*(int *)((int)this + 0xc) + 0xec),param_1);
  if (piVar1 != (int *)0x0) {
    iVar2 = (**(code **)(*piVar1 + 0x18))();
    if (iVar2 == 3) {
      uVar3 = FUN_004b1650((int)this);
    }
    else if (iVar2 == 4) {
      uVar3 = (**(code **)(*(int *)this + 0x4c))();
    }
    else if (iVar2 == 5) {
      uVar3 = FUN_004b16d0((int)this);
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return uVar3;
}

