
int __fastcall FUN_004dd050(int param_1)

{
  int iVar1;
  void *this;
  undefined4 *puVar2;
  void *pvVar3;
  int iVar4;
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063d4c8;
  local_c = ExceptionList;
  iVar4 = 0;
  ExceptionList = &local_c;
  for (this = (void *)thunk_FUN_005f5060(param_1 + 0x58); this != (void *)0x0;
      this = *(void **)((int)this + 0x10)) {
    puVar2 = (undefined4 *)FUN_00403040(this,local_14);
    iVar1 = *(int *)(param_1 + 0x2c);
    local_10 = &stack0xffffffd8;
    local_4 = 0;
    FUN_004f26d0(&stack0xffffffd8,puVar2);
    pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
    local_4 = 0xffffffff;
    FUN_00619730();
    if (pvVar3 != (void *)0x0) {
      iVar4 = iVar4 + *(int *)((int)pvVar3 + 0xd8);
    }
  }
  iVar1 = *(int *)(*(int *)(param_1 + 0x2c) + 0x184);
  iVar4 = (*(int *)(param_1 + 0x24) * iVar1) / 100 - iVar4;
  if (iVar1 - *(int *)(*(int *)(param_1 + 0x2c) + 0x188) < iVar4) {
    iVar4 = 0;
  }
  ExceptionList = local_c;
  return iVar4;
}

