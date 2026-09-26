
void __fastcall FUN_00477cb0(int param_1)

{
  int *piVar1;
  void *this;
  undefined4 *puVar2;
  void *pvVar3;
  int iVar4;
  int *piVar5;
  undefined1 local_14 [4];
  undefined1 *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633148;
  local_c = ExceptionList;
  piVar1 = (int *)(param_1 + 0x6c);
  piVar5 = piVar1;
  ExceptionList = &local_c;
  for (iVar4 = 10; iVar4 != 0; iVar4 = iVar4 + -1) {
    *piVar5 = 0;
    piVar5 = piVar5 + 1;
  }
  for (this = (void *)thunk_FUN_005f5060(param_1 + 0x2c); this != (void *)0x0;
      this = *(void **)((int)this + 0x10)) {
    puVar2 = FUN_00403040(this,local_14);
    iVar4 = *(int *)(param_1 + 0x58);
    local_10 = &stack0xffffffd8;
    local_4 = 0;
    FUN_004f26d0(&stack0xffffffd8,puVar2);
    pvVar3 = FUN_00419e40((void *)(iVar4 + 0x78));
    local_4 = 0xffffffff;
    FUN_00619730();
    if (pvVar3 != (void *)0x0) {
      *piVar1 = *piVar1 + *(int *)((int)pvVar3 + 0xb4);
      *(int *)(param_1 + 0x70) = *(int *)(param_1 + 0x70) + *(int *)((int)pvVar3 + 0xb8);
      *(int *)(param_1 + 0x74) = *(int *)(param_1 + 0x74) + *(int *)((int)pvVar3 + 0x84);
      *(int *)(param_1 + 0x78) = *(int *)(param_1 + 0x78) + *(int *)((int)pvVar3 + 0x78);
      *(int *)(param_1 + 0x80) = *(int *)(param_1 + 0x80) + *(int *)((int)pvVar3 + 0x68);
      *(int *)(param_1 + 0x88) = *(int *)(param_1 + 0x88) + *(int *)((int)pvVar3 + 0x7c);
      *(int *)(param_1 + 0x8c) = *(int *)(param_1 + 0x8c) + *(int *)((int)pvVar3 + 0x70);
      *(int *)(param_1 + 0x90) = *(int *)(param_1 + 0x90) + *(int *)((int)pvVar3 + 0x9c);
      if ((*(byte *)((int)pvVar3 + 0x38) & 0x10) != 0) {
        *(int *)(param_1 + 0x7c) = *(int *)(param_1 + 0x7c) + 1;
      }
      if ((*(byte *)((int)pvVar3 + 0x38) & 0x20) != 0) {
        *(int *)(param_1 + 0x84) = *(int *)(param_1 + 0x84) + 1;
      }
    }
  }
  if (0 < *piVar1) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x200000;
  }
  if (0 < *(int *)(param_1 + 0x70)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x400000;
  }
  if (0 < *(int *)(param_1 + 0x74)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x800000;
  }
  if (0 < *(int *)(param_1 + 0x78)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x1000000;
  }
  if (0 < *(int *)(param_1 + 0x80)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x2000000;
  }
  if (0 < *(int *)(param_1 + 0x88)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x4000000;
  }
  if (0 < *(int *)(param_1 + 0x8c)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x8000000;
  }
  if (0 < *(int *)(param_1 + 0x90)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x10000000;
  }
  if (0 < *(int *)(param_1 + 0x84)) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x100000;
  }
  *(undefined4 *)(param_1 + 0x94) = *(undefined4 *)(param_1 + 0x74);
  *(undefined4 *)(param_1 + 0x98) = *(undefined4 *)(param_1 + 0x78);
  ExceptionList = local_c;
  return;
}

