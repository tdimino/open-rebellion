
void __fastcall FUN_004e5560(int param_1)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  int iVar4;
  void *pvVar5;
  int iVar6;
  uint *puVar7;
  bool bVar8;
  int local_1c;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e348;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950(&local_10,0);
  local_4 = 0;
  bVar1 = true;
  bVar2 = false;
  bVar8 = false;
  bVar3 = false;
  local_1c = 0;
  *(undefined4 *)(param_1 + 0x14) = 0;
  *(undefined4 *)(param_1 + 0x18) = 0;
  iVar4 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c));
  do {
    if (iVar4 == 0) {
      if (bVar1) {
        for (iVar4 = thunk_FUN_005f5060(*(int *)(param_1 + 0x2c)); iVar4 != 0;
            iVar4 = *(int *)(iVar4 + 0x10)) {
          pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),(uint *)(iVar4 + 0x1c));
          if ((pvVar5 != (void *)0x0) &&
             (iVar6 = FUN_005f5500((void *)(*(int *)(param_1 + 0xc) + 0xd8),
                                   *(uint *)((int)pvVar5 + 0x38)), iVar6 != 0)) {
            local_10 = *(uint *)(iVar6 + 0x60);
            if ((local_10 & 0x80000) == 0) {
              bVar3 = true;
            }
            bVar8 = (local_10 & 0x10000) != 0;
            if (bVar8) {
              bVar2 = true;
              local_1c = local_1c + 1;
            }
            if ((local_10 & 0x20000) == 0) {
              puVar7 = (uint *)(*(int *)(param_1 + 0xc) + 4);
              *puVar7 = *puVar7 | 2;
            }
            *(uint *)(iVar6 + 0x60) = *(uint *)(iVar6 + 0x60) & 0xf7ffffff;
          }
        }
        if (bVar2) {
          if ((bVar8) && (1 < local_1c)) {
            puVar7 = (uint *)(*(int *)(param_1 + 0xc) + 4);
            *puVar7 = *puVar7 | 2;
            *(undefined4 *)(param_1 + 0x18) = 1;
          }
        }
        else if ((!bVar3) && ((*(byte *)(param_1 + 0x10) & 1) == 0)) {
          puVar7 = (uint *)(*(int *)(param_1 + 0xc) + 4);
          *puVar7 = *puVar7 | 2;
          *(undefined4 *)(param_1 + 0x14) = 1;
        }
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    }
    pvVar5 = FUN_004f5940((void *)(*(int *)(param_1 + 0xc) + 0x44),(uint *)(iVar4 + 0x1c));
    if (pvVar5 != (void *)0x0) {
      pvVar5 = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0xc) + 0xd8),
                                    *(uint *)((int)pvVar5 + 0x38));
      if (pvVar5 != (void *)0x0) {
        FUN_004769f0(pvVar5,*(int *)(iVar4 + 0x18));
        if (((*(uint *)((int)pvVar5 + 0x60) & 0x8000000) != 0) &&
           ((*(uint *)((int)pvVar5 + 0x60) & 0x4000000) != 0)) goto LAB_004e5602;
      }
      bVar1 = false;
    }
LAB_004e5602:
    iVar4 = *(int *)(iVar4 + 0x10);
  } while( true );
}

