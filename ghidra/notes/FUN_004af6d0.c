
void __fastcall FUN_004af6d0(int param_1)

{
  uint *puVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  void *pvVar6;
  int *piVar7;
  uint uVar8;
  int iVar9;
  uint local_24;
  uint local_20;
  undefined1 local_1c [4];
  undefined1 *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638c38;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_4 = 0;
  FUN_005f4950(&local_20,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),
                        (uint *)(*(int *)(param_1 + 0x10) + 0x34));
  if (pvVar3 != (void *)0x0) {
    piVar7 = (int *)((int)pvVar3 + 0x48);
    iVar9 = 10;
    do {
      if (*piVar7 != 0) {
        puVar1 = (uint *)(*piVar7 + 0x30);
        *puVar1 = *puVar1 & 0xfe0fffff;
      }
      piVar7 = piVar7 + 1;
      iVar9 = iVar9 + -1;
    } while (iVar9 != 0);
    for (pvVar3 = (void *)thunk_FUN_005f5060(*(int *)(param_1 + 0x10) + 0x2c); pvVar3 != (void *)0x0
        ; pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
      puVar4 = FUN_00403040(pvVar3,local_1c);
      iVar9 = *(int *)(param_1 + 0xc);
      local_18 = &stack0xffffffc8;
      local_4._0_1_ = 2;
      FUN_004f26d0(&stack0xffffffc8,puVar4);
      pvVar5 = FUN_00419e40((void *)(iVar9 + 0x78));
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      if (pvVar5 != (void *)0x0) {
        FUN_004f26d0(&local_24,(undefined4 *)((int)pvVar5 + 0x2c));
        local_14 = 0x90;
        local_10 = 0x98;
        if ((local_24 >> 0x18 < 0x90) || (0x97 < local_24 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          iVar9 = *(int *)(param_1 + 0xc);
          local_18 = &stack0xffffffc8;
          FUN_004f26d0(&stack0xffffffc8,&local_24);
          pvVar6 = FUN_00403d30((void *)(iVar9 + 0x2c));
          if (pvVar6 != (void *)0x0) {
            local_20 = *(uint *)((int)pvVar5 + 0x38);
            if ((local_20 & 0x80000000) == 0) {
              if ((local_20 & 0x10000000) == 0) {
                if ((local_20 & 0x20000000) == 0) {
                  if ((local_20 & 0x40000000) == 0) goto LAB_004af857;
                  uVar8 = *(uint *)((int)pvVar6 + 0x30) | 0x400000;
                }
                else {
                  uVar8 = *(uint *)((int)pvVar6 + 0x30) | 0x200000;
                }
              }
              else {
                uVar8 = *(uint *)((int)pvVar6 + 0x30) | 0x100000;
              }
            }
            else {
              uVar8 = *(uint *)((int)pvVar6 + 0x30) | 0x1000000;
            }
            *(uint *)((int)pvVar6 + 0x30) = uVar8;
          }
        }
      }
LAB_004af857:
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

