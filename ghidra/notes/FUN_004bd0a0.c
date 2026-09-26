
undefined4 __fastcall FUN_004bd0a0(int param_1)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  void *pvVar5;
  uint *puVar6;
  void *pvVar7;
  int *this;
  undefined4 *puVar8;
  uint local_44;
  undefined4 local_40;
  uint local_3c;
  undefined1 local_38 [4];
  uint local_34;
  undefined1 *local_30;
  undefined1 *local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639d18;
  local_c = ExceptionList;
  bVar2 = false;
  local_40 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_44);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xfffffffb;
  if ((*(byte *)(param_1 + 0x20) & 1) == 0) {
    FUN_004ece80((undefined4 *)(param_1 + 0x38));
LAB_004bd531:
    if ((*(byte *)(param_1 + 0x20) & 4) == 0) goto LAB_004bd53f;
  }
  else {
    puVar8 = (undefined4 *)(param_1 + 0x38);
    uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
    local_2c = (undefined1 *)0x90;
    local_28 = 0x98;
    if ((0x8f < uVar3) && (uVar3 < 0x98)) {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_30 = &stack0xffffffa4;
      FUN_004f26d0(&stack0xffffffa4,puVar8);
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar4 != (void *)0x0) {
        if (((*(byte *)((int)pvVar4 + 0x30) & 1) == 0) || ((*(byte *)((int)pvVar4 + 0x28) & 2) != 0)
           ) {
          FUN_004ece80(puVar8);
        }
        else {
          *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 4;
        }
      }
    }
    if ((*(byte *)(param_1 + 0x20) & 4) == 0) {
      uVar3 = *(uint *)(param_1 + 0x34) >> 0x18;
      local_2c = (undefined1 *)0x90;
      local_28 = 0x98;
      if ((uVar3 < 0x90) || (0x97 < uVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        iVar1 = *(int *)(param_1 + 0x68);
        local_30 = &stack0xffffffa4;
        FUN_004f26d0(&stack0xffffffa4,(undefined4 *)(param_1 + 0x34));
        pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
        if (pvVar4 != (void *)0x0) {
          if (((*(byte *)((int)pvVar4 + 0x30) & 1) == 0) ||
             ((*(byte *)((int)pvVar4 + 0x28) & 2) != 0)) {
            puVar8 = FUN_004bd7d0(pvVar4,&local_3c);
            local_4._0_1_ = 1;
            pvVar5 = FUN_0041a460(*(void **)(param_1 + 0x68),puVar8,0,0,1,2,0,0,(void *)0x1);
            local_4._0_1_ = 0;
            FUN_00619730();
            pvVar7 = (void *)(param_1 + 0x54);
            FUN_00435790(pvVar7,(int)pvVar5);
            puVar8 = FUN_004357b0(pvVar7,&local_3c);
            local_4._0_1_ = 2;
            FUN_004f26d0((uint *)(param_1 + 0x38),puVar8);
            local_4._0_1_ = 0;
            FUN_00619730();
            FUN_005f58b0(pvVar7);
            local_2c = (undefined1 *)0x90;
            uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
            local_28 = 0x98;
            if ((uVar3 < 0x90) || (0x97 < uVar3)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if (!bVar2) {
              puVar6 = FUN_004bd7d0(pvVar4,local_38);
              local_4._0_1_ = 3;
              puVar8 = FUN_0041b2b0(*(void **)(param_1 + 0x68),&local_3c,puVar6,0,0,1,0,0,0x200);
              local_4._0_1_ = 4;
              FUN_004f26d0((void *)(param_1 + 0x38),puVar8);
              local_4._0_1_ = 3;
              FUN_00619730();
              local_4._0_1_ = 0;
              FUN_00619730();
            }
          }
          else {
            FUN_004f26d0(puVar8,(undefined4 *)(param_1 + 0x34));
          }
        }
      }
      local_2c = (undefined1 *)0x90;
      uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
      local_28 = 0x98;
      if ((uVar3 < 0x90) || (0x97 < uVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 4;
      }
      if ((*(byte *)(param_1 + 0x20) & 4) == 0) {
        for (pvVar4 = (void *)thunk_FUN_005f5060(param_1 + 0x3c); pvVar4 != (void *)0x0;
            pvVar4 = *(void **)((int)pvVar4 + 0x10)) {
          if ((*(byte *)(param_1 + 0x20) & 4) != 0) goto LAB_004bd537;
          puVar6 = FUN_00403040(pvVar4,local_38);
          local_4._0_1_ = 5;
          pvVar7 = FUN_004f5940(*(void **)(param_1 + 100),puVar6);
          local_4._0_1_ = 0;
          FUN_00619730();
          if (pvVar7 != (void *)0x0) {
            FUN_004f26d0(&local_44,(undefined4 *)((int)pvVar7 + 0x38));
            local_24 = 0x90;
            local_20 = 0x98;
            if ((local_44 >> 0x18 < 0x90) || (0x97 < local_44 >> 0x18)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if ((!bVar2) &&
               (this = FUN_004f2d10(*(int *)(param_1 + 0x24),&local_44), this != (int *)0x0)) {
              do {
                puVar6 = FUN_004025b0(this,&local_3c);
                local_1c = 0x90;
                local_18 = 0x98;
                local_4._0_1_ = 6;
                if ((*puVar6 >> 0x18 < 0x90) || (0x97 < *puVar6 >> 0x18)) {
                  bVar2 = false;
                }
                else {
                  bVar2 = true;
                }
                FUN_00619730();
                local_4._0_1_ = 0;
                FUN_00619730();
              } while ((!bVar2) && (this = (int *)this[7], this != (int *)0x0));
              if (this != (int *)0x0) {
                puVar6 = FUN_004025b0(this,&local_34);
                local_4._0_1_ = 7;
                FUN_004f26d0(&local_44,puVar6);
                local_4._0_1_ = 0;
                FUN_00619730();
              }
            }
            local_14 = 0x90;
            local_10 = 0x98;
            if ((local_44 >> 0x18 < 0x90) || (0x97 < local_44 >> 0x18)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if (bVar2) {
              iVar1 = *(int *)(param_1 + 0x68);
              local_2c = &stack0xffffffa4;
              FUN_004f26d0(&stack0xffffffa4,&local_44);
              pvVar7 = FUN_00403d30((void *)(iVar1 + 0x2c));
              if (((pvVar7 != (void *)0x0) && ((*(byte *)((int)pvVar7 + 0x30) & 1) != 0)) &&
                 ((*(byte *)((int)pvVar7 + 0x28) & 2) == 0)) {
                *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 4;
                puVar8 = FUN_00403040(pvVar7,&local_30);
                local_4._0_1_ = 8;
                FUN_004f26d0((void *)(param_1 + 0x38),puVar8);
                local_4._0_1_ = 0;
                FUN_00619730();
              }
            }
          }
        }
        goto LAB_004bd531;
      }
    }
  }
LAB_004bd537:
  local_40 = 1;
LAB_004bd53f:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_40;
}

