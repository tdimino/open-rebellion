
void __fastcall FUN_004742b0(void *param_1)

{
  void *pvVar1;
  int iVar2;
  bool bVar3;
  void *pvVar4;
  uint *puVar5;
  uint uVar6;
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632a68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar4 = (void *)thunk_FUN_005f5060((int)param_1 + 0x24);
  while (pvVar4 != (void *)0x0) {
    pvVar1 = *(void **)((int)pvVar4 + 0x10);
    puVar5 = FUN_00403040(pvVar4,&local_18);
    local_4 = 0;
    FUN_00475f60(param_1,puVar5);
    local_4 = 0xffffffff;
    FUN_00619730();
    pvVar4 = pvVar1;
  }
  FUN_00476080((int)param_1);
  uVar6 = *(uint *)((int)param_1 + 0x30) >> 0x18;
  local_14 = 0x90;
  local_10 = 0x98;
  if ((uVar6 < 0x90) || (0x97 < uVar6)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar2 = *(int *)((int)param_1 + 0x4c);
    local_18 = &stack0xffffffd4;
    FUN_004f26d0(&stack0xffffffd4,(undefined4 *)((int)param_1 + 0x30));
    pvVar4 = FUN_00403d30((void *)(iVar2 + 0x2c));
    if (pvVar4 != (void *)0x0) {
      FUN_004ece80((undefined4 *)((int)param_1 + 0x30));
      *(uint *)((int)pvVar4 + 0x30) = *(uint *)((int)pvVar4 + 0x30) & 0xfff7ffff;
    }
  }
  ExceptionList = local_c;
  return;
}

