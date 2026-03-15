
void __thiscall FUN_004ae480(void *this,uint *param_1,undefined4 *param_2,uint param_3)

{
  bool bVar1;
  ushort uVar2;
  void *pvVar3;
  undefined1 *puVar4;
  uint *puVar5;
  void *pvVar6;
  undefined4 *puVar7;
  uint uVar8;
  int iVar9;
  undefined1 *puVar10;
  int *piVar11;
  undefined1 **ppuVar12;
  undefined1 *local_90;
  uint local_8c;
  undefined1 *local_88;
  int *local_84;
  undefined4 local_80 [2];
  undefined4 auStack_78 [7];
  int iStack_5c;
  void *local_58;
  undefined4 local_54;
  undefined4 local_50 [7];
  int local_34;
  void *local_30;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638b6f;
  pvStack_c = ExceptionList;
  puVar10 = (undefined1 *)0x0;
  bVar1 = false;
  local_88 = (undefined1 *)0x0;
  ExceptionList = &pvStack_c;
  local_30 = this;
  FUN_004f5510(local_80);
  local_4 = 0;
  FUN_004ece80(param_2);
  local_58 = (void *)0x90;
  local_54 = 0x98;
  if ((0x8f < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x98)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    local_84 = FUN_004f3220(*(int *)((int)this + 4),param_1);
    iVar9 = *(int *)((int)this + 0xc);
    local_90 = &stack0xffffff5c;
    FUN_004f26d0(&stack0xffffff5c,param_1);
    pvVar3 = FUN_00403d30((void *)(iVar9 + 0x2c));
    piVar11 = local_84;
    if ((local_84 != (int *)0x0) && (pvVar3 != (void *)0x0)) {
      if ((param_3 & 0x2000000) != 0) {
        FUN_00526cf0(local_50,local_84,1);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00513120((int)local_50);
        while (local_34 != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 2;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)local_50);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)0x1;
            local_88 = (undefined1 *)0x1;
            local_4 = CONCAT31(local_4._1_3_,3);
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 2;
          if (puVar10 != (undefined1 *)0x0) {
            puVar10 = (undefined1 *)0x0;
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,1);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)local_50);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00526df0(local_50);
      }
      if ((param_3 & 0x40000) != 0) {
        FUN_005278c0(local_50,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_00513120((int)local_50);
        while (local_34 != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 5;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)local_50);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)((uint)puVar10 | 2);
            local_4 = CONCAT31(local_4._1_3_,6);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 5;
          if (((uint)puVar10 & 2) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xfffffffd);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,4);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)local_50);
          piVar11 = local_84;
        }
        local_4 = local_4 & 0xffffff00;
        FUN_005279c0(local_50);
      }
      if ((param_3 & 0x80000) != 0) {
        FUN_0051c270(local_50,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,7);
        FUN_00513120((int)local_50);
        while (local_34 != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 8;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)local_50);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)((uint)puVar10 | 4);
            local_4 = CONCAT31(local_4._1_3_,9);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 8;
          if (((uint)puVar10 & 4) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xfffffffb);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,7);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)local_50);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_0051c2f0(local_50);
      }
      if ((param_3 & 0x100000) != 0) {
        FUN_005272e0(auStack_78,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,10);
        FUN_00513120((int)auStack_78);
        while (iStack_5c != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0xb;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)auStack_78);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)((uint)puVar10 | 8);
            local_4 = CONCAT31(local_4._1_3_,0xc);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 0xb;
          if (((uint)puVar10 & 8) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xfffffff7);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,10);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)auStack_78);
        }
        FUN_00527550(local_50,local_84,1);
        local_4 = CONCAT31(local_4._1_3_,0xd);
        FUN_00513120((int)local_50);
        while (local_34 != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0xe;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)local_50);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x10);
            local_4 = CONCAT31(local_4._1_3_,0xf);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 0xe;
          if (((uint)puVar10 & 0x10) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xffffffef);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,0xd);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)local_50);
        }
        local_4._1_3_ = (uint3)(local_4 >> 8);
        local_4._0_1_ = 10;
        FUN_00527650(local_50);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00527360(auStack_78);
        piVar11 = local_84;
      }
      if ((param_3 & 0x200000) != 0) {
        FUN_00504c40(auStack_78,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,0x10);
        FUN_00513120((int)auStack_78);
        while (iStack_5c != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0x11;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)auStack_78);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x20);
            local_4 = CONCAT31(local_4._1_3_,0x12);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 0x11;
          if (((uint)puVar10 & 0x20) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xffffffdf);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,0x10);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)auStack_78);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00504d40(auStack_78);
        piVar11 = local_84;
      }
      if ((param_3 & 0x400000) != 0) {
        FUN_005039d0(auStack_78,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,0x13);
        FUN_00513120((int)auStack_78);
        while (iStack_5c != 0) {
          puVar4 = (undefined1 *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0x14;
          local_90 = puVar4;
          if (puVar4 == (undefined1 *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            puVar5 = &local_8c;
            pvVar3 = (void *)FUN_0052bed0((int)auStack_78);
            puVar5 = FUN_004025b0(pvVar3,puVar5);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x40);
            local_4 = CONCAT31(local_4._1_3_,0x15);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(puVar4,puVar5);
          }
          local_4 = 0x14;
          if (((uint)puVar10 & 0x40) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xffffffbf);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,0x13);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)auStack_78);
        }
        local_4 = local_4 & 0xffffff00;
        FUN_00503ad0(auStack_78);
        piVar11 = local_84;
      }
      if ((param_3 & 0x800000) != 0) {
        FUN_004ffe70(auStack_78,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,0x16);
        FUN_00513120((int)auStack_78);
        while (iStack_5c != 0) {
          puVar5 = &local_8c;
          pvVar3 = (void *)FUN_0052bed0((int)auStack_78);
          puVar5 = FUN_004025b0(pvVar3,puVar5);
          local_4._0_1_ = 0x17;
          iVar9 = *(int *)((int)local_30 + 0xc);
          local_88 = &stack0xffffff5c;
          FUN_004f26d0(&stack0xffffff5c,puVar5);
          pvVar3 = FUN_00419e40((void *)(iVar9 + 0x78));
          local_4 = CONCAT31(local_4._1_3_,0x16);
          FUN_00619730();
          if ((pvVar3 != (void *)0x0) && ((*(byte *)((int)pvVar3 + 0x38) & 1) == 0)) {
            for (pvVar3 = (void *)thunk_FUN_005f5060((int)pvVar3 + 0x48); pvVar3 != (void *)0x0;
                pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
              pvVar6 = (void *)FUN_00618b70(0x1c);
              local_4._0_1_ = 0x18;
              local_58 = pvVar6;
              if (pvVar6 == (void *)0x0) {
                pvVar6 = (void *)0x0;
              }
              else {
                puVar7 = (undefined4 *)FUN_00403040(pvVar3,&local_90);
                puVar10 = (undefined1 *)((uint)puVar10 | 0x80);
                local_4 = CONCAT31(local_4._1_3_,0x19);
                local_88 = puVar10;
                pvVar6 = FUN_004ecc70(pvVar6,puVar7);
              }
              local_4 = 0x18;
              if (((uint)puVar10 & 0x80) != 0) {
                puVar10 = (undefined1 *)((uint)puVar10 & 0xffffff7f);
                FUN_00619730();
              }
              local_4 = CONCAT31(local_4._1_3_,0x16);
              if (pvVar6 != (void *)0x0) {
                FUN_004f57b0(local_80,pvVar6);
              }
            }
          }
          FUN_005130d0((int)auStack_78);
          piVar11 = local_84;
        }
        local_4 = local_4 & 0xffffff00;
        FUN_004fff70(auStack_78);
      }
      if ((param_3 & 0x20000) != 0) {
        FUN_0052c740(auStack_2c,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,0x1a);
        FUN_00513120((int)auStack_2c);
        while (iStack_10 != 0) {
          pvVar3 = (void *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0x1b;
          local_58 = pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            ppuVar12 = &local_90;
            pvVar6 = (void *)FUN_0052bed0((int)auStack_2c);
            puVar5 = FUN_004025b0(pvVar6,(uint *)ppuVar12);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x100);
            local_4 = CONCAT31(local_4._1_3_,0x1c);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(pvVar3,puVar5);
          }
          local_4 = 0x1b;
          if (((uint)puVar10 & 0x100) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xfffffeff);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,0x1a);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)auStack_2c);
        }
        FUN_0052cab0(local_50,local_84,1);
        local_4 = CONCAT31(local_4._1_3_,0x1d);
        FUN_00513120((int)local_50);
        while (local_34 != 0) {
          pvVar3 = (void *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0x1e;
          local_58 = pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            ppuVar12 = &local_90;
            pvVar6 = (void *)FUN_0052bed0((int)local_50);
            puVar5 = FUN_004025b0(pvVar6,(uint *)ppuVar12);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x200);
            local_4 = CONCAT31(local_4._1_3_,0x1f);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(pvVar3,puVar5);
          }
          local_4 = 0x1e;
          if (((uint)puVar10 & 0x200) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xfffffdff);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,0x1d);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)local_50);
        }
        uVar8 = FUN_005f4960((undefined4 *)(*(int *)((int)local_30 + 0xc) + 4));
        if ((uVar8 & 0x40) == 0) {
          FUN_0052c420(auStack_78,local_84,1);
          local_4 = CONCAT31(local_4._1_3_,0x20);
          FUN_00513120((int)auStack_78);
          while (iStack_5c != 0) {
            pvVar3 = (void *)FUN_00618b70(0x1c);
            local_4._0_1_ = 0x21;
            local_58 = pvVar3;
            if (pvVar3 == (void *)0x0) {
              pvVar3 = (void *)0x0;
            }
            else {
              ppuVar12 = &local_90;
              pvVar6 = (void *)FUN_0052bed0((int)auStack_78);
              puVar5 = FUN_004025b0(pvVar6,(uint *)ppuVar12);
              puVar10 = (undefined1 *)((uint)puVar10 | 0x400);
              local_4 = CONCAT31(local_4._1_3_,0x22);
              local_88 = puVar10;
              pvVar3 = FUN_004ecc70(pvVar3,puVar5);
            }
            local_4 = 0x21;
            if (((uint)puVar10 & 0x400) != 0) {
              puVar10 = (undefined1 *)((uint)puVar10 & 0xfffffbff);
              FUN_00619730();
            }
            local_4 = CONCAT31(local_4._1_3_,0x20);
            if (pvVar3 != (void *)0x0) {
              FUN_004f57b0(local_80,pvVar3);
            }
            FUN_005130d0((int)auStack_78);
          }
          local_4._1_3_ = (uint3)(local_4 >> 8);
          local_4 = CONCAT31(local_4._1_3_,0x1d);
          FUN_0052c520(auStack_78);
        }
        local_4._0_1_ = 0x1a;
        FUN_0052cbb0(local_50);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_0052c840(auStack_2c);
        piVar11 = local_84;
      }
      if ((param_3 & 0x10000) != 0) {
        FUN_0052d1f0(auStack_2c,piVar11,1);
        local_4 = CONCAT31(local_4._1_3_,0x23);
        FUN_00513120((int)auStack_2c);
        while (iStack_10 != 0) {
          pvVar3 = (void *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0x24;
          local_58 = pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            ppuVar12 = &local_90;
            pvVar6 = (void *)FUN_0052bed0((int)auStack_2c);
            puVar5 = FUN_004025b0(pvVar6,(uint *)ppuVar12);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x800);
            local_4 = CONCAT31(local_4._1_3_,0x25);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(pvVar3,puVar5);
          }
          local_4 = 0x24;
          if (((uint)puVar10 & 0x800) != 0) {
            puVar10 = (undefined1 *)((uint)puVar10 & 0xfffff7ff);
            FUN_00619730();
          }
          local_4 = CONCAT31(local_4._1_3_,0x23);
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)auStack_2c);
        }
        FUN_0052d610(auStack_78,local_84,1);
        local_4._0_1_ = 0x26;
        FUN_00513120((int)auStack_78);
        if (iStack_5c != 0) {
          pvVar3 = (void *)FUN_00618b70(0x1c);
          local_4._0_1_ = 0x27;
          local_58 = pvVar3;
          if (pvVar3 == (void *)0x0) {
            pvVar3 = (void *)0x0;
          }
          else {
            ppuVar12 = &local_90;
            pvVar6 = (void *)FUN_0052bed0((int)auStack_78);
            puVar5 = FUN_004025b0(pvVar6,(uint *)ppuVar12);
            puVar10 = (undefined1 *)((uint)puVar10 | 0x1000);
            local_4 = CONCAT31(local_4._1_3_,0x28);
            local_88 = puVar10;
            pvVar3 = FUN_004ecc70(pvVar3,puVar5);
          }
          local_4 = 0x27;
          if (((uint)puVar10 & 0x1000) != 0) {
            FUN_00619730();
          }
          local_4._0_1_ = 0x26;
          if (pvVar3 != (void *)0x0) {
            FUN_004f57b0(local_80,pvVar3);
          }
          FUN_005130d0((int)auStack_78);
        }
        local_4._0_1_ = 0x23;
        FUN_0052d710(auStack_78);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_0052d2f0(auStack_2c);
      }
      uVar2 = FUN_005f50e0((int)local_80);
      if (uVar2 != 0) {
        iVar9 = FUN_0041cd80((uint)uVar2);
        pvVar3 = (void *)thunk_FUN_005f5060((int)local_80);
        if (pvVar3 != (void *)0x0) {
          do {
            if (iVar9 < 1) break;
            pvVar3 = *(void **)((int)pvVar3 + 0x10);
            iVar9 = iVar9 + -1;
          } while (pvVar3 != (void *)0x0);
          if (pvVar3 != (void *)0x0) {
            puVar7 = (undefined4 *)FUN_00403040(pvVar3,&local_90);
            local_4._0_1_ = 0x29;
            FUN_004f26d0(param_2,puVar7);
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
          }
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f55c0(local_80);
  ExceptionList = pvStack_c;
  return;
}

