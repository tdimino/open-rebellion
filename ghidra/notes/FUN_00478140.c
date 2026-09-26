
void __fastcall FUN_00478140(void *param_1)

{
  int iVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633188;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = param_1;
  FUN_005f4950(&local_10,0);
  *(undefined4 *)((int)param_1 + 0xe0) = 0;
  *(undefined4 *)((int)param_1 + 0xe4) = 0;
  local_4 = 0;
  *(uint *)((int)param_1 + 0x60) = *(uint *)((int)param_1 + 0x60) & 0xf8ffffff;
  for (iVar1 = thunk_FUN_005f5060((int)param_1 + 0xd4); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0x10))
  {
    piVar2 = (int *)FUN_005f5500((void *)(*(int *)((int)param_1 + 0x58) + 0xec),
                                 *(uint *)(iVar1 + 0x18));
    if (piVar2 != (int *)0x0) {
      local_10 = (void *)piVar2[8];
      *(uint *)((int)param_1 + 0x60) = *(uint *)((int)param_1 + 0x60) | 0x2000000;
      *(uint *)((int)param_1 + 100) = *(uint *)((int)param_1 + 100) | 0x40000000;
      if (((uint)local_10 & 0x10) != 0) {
        *(uint *)((int)param_1 + 0x60) = *(uint *)((int)param_1 + 0x60) | 0x4000000;
      }
      if (((uint)local_10 & 8) != 0) {
        if (((uint)local_10 & 4) == 0) {
          *(uint *)((int)param_1 + 0x60) = *(uint *)((int)param_1 + 0x60) | 0x1000000;
        }
        else {
          iVar4 = (**(code **)(*piVar2 + 0x18))();
          if (iVar4 == 4) {
            *(uint *)((int)param_1 + 0x60) = *(uint *)((int)param_1 + 0x60) | 0x200000;
          }
        }
      }
      if (((uint)local_10 & 1) == 0) {
        uVar3 = *(uint *)((int)param_1 + 100) | 0x400;
      }
      else {
        uVar3 = *(uint *)((int)param_1 + 100) | 0x80000000;
      }
      *(uint *)((int)param_1 + 100) = uVar3;
    }
  }
  (**(code **)(**(int **)((int)param_1 + 0x5c) + 0x1c))((int *)((int)param_1 + 0xe0));
  if (*(int *)((int)param_1 + 0xe0) != 0) {
    *(uint *)((int)param_1 + 100) = *(uint *)((int)param_1 + 100) | 0x200;
  }
  iVar1 = thunk_FUN_005f5060((int)param_1 + 0xd4);
  while ((iVar1 != 0 && (*(int *)((int)param_1 + 0xe4) == 0))) {
    iVar4 = FUN_004add40(*(void **)((int)param_1 + 0x5c),*(uint *)(iVar1 + 0x18));
    if (iVar4 != 0) {
      *(undefined4 *)((int)param_1 + 0xe4) = *(undefined4 *)(iVar1 + 0x18);
    }
    iVar1 = *(int *)(iVar1 + 0x10);
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_10;
  return;
}

