
int FUN_00549850(int param_1,undefined4 param_2)

{
  void *this;
  uint *puVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00647c78;
  local_c = ExceptionList;
  iVar4 = 1;
  ExceptionList = &local_c;
  for (this = (void *)thunk_FUN_005f5060(param_1); this != (void *)0x0;
      this = *(void **)((int)this + 0x10)) {
    puVar1 = FUN_00403040(this,&param_1);
    local_4 = 0;
    piVar2 = (int *)FUN_00505d40(puVar1);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (((piVar2 != (int *)0x0) && (iVar3 = (**(code **)(*piVar2 + 0x1d4))(), iVar3 != 0)) &&
       ((*(byte *)(piVar2 + 0x14) & 8) == 0)) {
      iVar3 = (**(code **)(*piVar2 + 0x218))(param_2);
      if ((iVar3 == 0) || (iVar4 == 0)) {
        iVar4 = 0;
      }
      else {
        iVar4 = 1;
      }
    }
  }
  ExceptionList = local_c;
  return iVar4;
}

