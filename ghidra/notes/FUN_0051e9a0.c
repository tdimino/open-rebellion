
bool __thiscall FUN_0051e9a0(void *this,int param_1,int param_2,int param_3)

{
  int *piVar1;
  bool bVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643320;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar1 = (int *)FUN_0051f730(2);
  bVar2 = false;
  if (piVar1 != (int *)0x0) {
    piVar1[6] = param_1;
    piVar1[3] = param_2;
    piVar1[5] = param_3;
    if (param_1 == 1) {
      param_1 = -0xcfffff1;
      local_4 = 0;
      FUN_004f26d0(piVar1 + 8,&param_1);
    }
    else {
      param_1 = -0xcfffff0;
      local_4 = 1;
      FUN_004f26d0(piVar1 + 8,&param_1);
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    bVar2 = *(int *)((int)this + 0xb8) != 0;
    if (bVar2) {
      (**(code **)(*piVar1 + 0x14))();
      FUN_00583c50(*(void **)((int)this + 0xb8),(int)piVar1);
      FUN_0051ebb0();
    }
  }
  ExceptionList = local_c;
  return bVar2;
}

