
void __thiscall FUN_00496770(void *this,uint param_1,uint *param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  int *piVar4;
  void *pvVar5;
  void *pvVar6;
  uint *puVar7;
  undefined4 uVar8;
  int iVar9;
  int *piVar10;
  undefined4 *puVar11;
  void *pvVar12;
  int iVar13;
  bool bVar14;
  uint uVar15;
  uint **ppuVar16;
  undefined4 uVar17;
  uint local_70;
  int *local_6c;
  uint local_68;
  uint local_64;
  uint local_60;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636428;
  local_c = ExceptionList;
  bVar3 = false;
  bVar1 = false;
  bVar2 = false;
  bVar14 = false;
  if (param_1 != 2) {
    if (param_1 == 0x182) {
      ExceptionList = &local_c;
      piVar4 = FUN_004f36a0(*(int *)((int)this + 0x20),param_2);
      if (piVar4 == (int *)0x0) {
        ExceptionList = local_c;
        return;
      }
      *(uint *)((int)this + 0x6c) = (uint)piVar4[9] >> 6 & 3;
      if (param_3 == (void *)0x0) {
        FUN_00502db0(local_4c,piVar4,1);
        local_4 = 7;
        FUN_00513120((int)local_4c);
        FUN_00513120((int)local_4c);
        if (local_30 != 0) {
          pvVar12 = (void *)((int)this + 0x78);
          do {
            pvVar5 = (void *)FUN_00618b70(0x1c);
            local_4._0_1_ = 8;
            param_3 = pvVar5;
            if (pvVar5 == (void *)0x0) {
              pvVar5 = (void *)0x0;
            }
            else {
              puVar7 = &local_68;
              pvVar6 = (void *)FUN_0052bed0((int)local_4c);
              puVar7 = FUN_004025b0(pvVar6,puVar7);
              bVar1 = true;
              local_4 = CONCAT31(local_4._1_3_,9);
              pvVar5 = FUN_004ecc70(pvVar5,puVar7);
            }
            local_4 = 8;
            if (bVar1) {
              bVar1 = false;
              FUN_00619730();
            }
            local_4._0_1_ = 7;
            FUN_004f57b0(pvVar12,pvVar5);
            uVar17 = 1;
            uVar8 = FUN_0052bed0((int)local_4c);
            FUN_004fcd00(local_2c,uVar8,uVar17);
            local_4 = CONCAT31(local_4._1_3_,10);
            FUN_00513120((int)local_2c);
            while (local_10 != 0) {
              pvVar5 = (void *)FUN_00618b70(0x1c);
              local_4._0_1_ = 0xb;
              param_3 = pvVar5;
              if (pvVar5 == (void *)0x0) {
                pvVar5 = (void *)0x0;
              }
              else {
                puVar7 = &local_64;
                pvVar6 = (void *)FUN_0052bed0((int)local_2c);
                puVar7 = FUN_004025b0(pvVar6,puVar7);
                bVar3 = true;
                local_4 = CONCAT31(local_4._1_3_,0xc);
                pvVar5 = FUN_004ecc70(pvVar5,puVar7);
              }
              local_4 = 0xb;
              if (bVar3) {
                bVar3 = false;
                FUN_00619730();
              }
              puVar7 = &local_60;
              local_4._0_1_ = 10;
              pvVar6 = (void *)FUN_0052bed0((int)local_2c);
              puVar7 = FUN_004025b0(pvVar6,puVar7);
              local_54 = 0x30;
              local_50 = 0x3c;
              local_4._0_1_ = 0xd;
              if ((*puVar7 >> 0x18 < 0x30) || (0x3b < *puVar7 >> 0x18)) {
                bVar14 = false;
              }
              else {
                bVar14 = true;
              }
              FUN_00619730();
              local_4 = CONCAT31(local_4._1_3_,10);
              FUN_00619730();
              pvVar6 = pvVar12;
              if ((bVar14) &&
                 (iVar9 = FUN_0052bed0((int)local_2c), (*(byte *)(iVar9 + 0xac) & 1) != 0)) {
                pvVar6 = (void *)((int)this + 0x90);
              }
              FUN_004f57b0(pvVar6,pvVar5);
              FUN_005130d0((int)local_2c);
            }
            local_4._1_3_ = (undefined3)(local_4 >> 8);
            local_4 = CONCAT31(local_4._1_3_,7);
            FUN_004fcd80(local_2c);
            FUN_005130d0((int)local_4c);
          } while (local_30 != 0);
        }
        local_4 = 0xffffffff;
        FUN_00502eb0(local_4c);
      }
      *(undefined4 *)((int)this + 0x40) = 1;
      ExceptionList = local_c;
      return;
    }
    if (param_1 != 0x183) {
      return;
    }
    ExceptionList = &local_c;
    piVar4 = FUN_004f36a0(*(int *)((int)this + 0x20),param_2);
    if (piVar4 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    if (param_3 == (void *)0x0) {
      local_6c = piVar4;
      FUN_00502db0(local_2c,piVar4,1);
      local_4 = 0;
      FUN_00513120((int)local_2c);
      FUN_00513120((int)local_2c);
      if (local_10 != 0) {
        pvVar12 = (void *)((int)this + 0x78);
        do {
          pvVar5 = (void *)FUN_00618b70(0x1c);
          local_4._0_1_ = 1;
          param_3 = pvVar5;
          if (pvVar5 == (void *)0x0) {
            pvVar5 = (void *)0x0;
          }
          else {
            ppuVar16 = &param_2;
            pvVar6 = (void *)FUN_0052bed0((int)local_2c);
            puVar7 = FUN_004025b0(pvVar6,(uint *)ppuVar16);
            bVar14 = true;
            local_4 = CONCAT31(local_4._1_3_,2);
            pvVar5 = FUN_004ecc70(pvVar5,puVar7);
          }
          local_4 = 1;
          if (bVar14) {
            bVar14 = false;
            FUN_00619730();
          }
          local_4 = local_4 & 0xffffff00;
          FUN_004f57b0(pvVar12,pvVar5);
          uVar17 = 1;
          uVar8 = FUN_0052bed0((int)local_2c);
          FUN_004fcd00(local_4c,uVar8,uVar17);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00513120((int)local_4c);
          while (local_30 != 0) {
            pvVar5 = (void *)FUN_00618b70(0x1c);
            local_4._0_1_ = 4;
            param_3 = pvVar5;
            if (pvVar5 == (void *)0x0) {
              pvVar5 = (void *)0x0;
            }
            else {
              puVar7 = &param_1;
              pvVar6 = (void *)FUN_0052bed0((int)local_4c);
              puVar7 = FUN_004025b0(pvVar6,puVar7);
              bVar2 = true;
              local_4 = CONCAT31(local_4._1_3_,5);
              pvVar5 = FUN_004ecc70(pvVar5,puVar7);
            }
            local_4 = 4;
            if (bVar2) {
              bVar2 = false;
              FUN_00619730();
            }
            puVar7 = &local_70;
            local_4._0_1_ = 3;
            pvVar6 = (void *)FUN_0052bed0((int)local_4c);
            puVar7 = FUN_004025b0(pvVar6,puVar7);
            local_5c = 0x30;
            local_58 = 0x3c;
            local_4._0_1_ = 6;
            if ((*puVar7 >> 0x18 < 0x30) || (0x3b < *puVar7 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            local_4 = CONCAT31(local_4._1_3_,3);
            FUN_00619730();
            pvVar6 = pvVar12;
            if ((bVar1) && (iVar9 = FUN_0052bed0((int)local_4c), (*(byte *)(iVar9 + 0xac) & 1) != 0)
               ) {
              pvVar6 = (void *)((int)this + 0x90);
            }
            FUN_004f57b0(pvVar6,pvVar5);
            FUN_005130d0((int)local_4c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_004fcd80(local_4c);
          FUN_005130d0((int)local_2c);
          piVar4 = local_6c;
        } while (local_10 != 0);
      }
      local_4 = 0xffffffff;
      FUN_00502eb0(local_2c);
    }
    *(uint *)((int)this + 0x6c) = (uint)piVar4[9] >> 6 & 3;
    ExceptionList = local_c;
    return;
  }
  if (*(int *)((int)this + 0x6c) == 1) {
    ExceptionList = &local_c;
    iVar9 = FUN_004f3dd0(*(int *)((int)this + 0x20),1);
  }
  else {
    ExceptionList = &local_c;
    iVar9 = FUN_004f3dd0(*(int *)((int)this + 0x20),2);
  }
  if (*(int *)((int)this + 0x68) == 1) {
    iVar13 = *(int *)((int)this + 0x20);
    uVar15 = 1;
  }
  else {
    if (*(int *)((int)this + 0x68) != 2) {
      iVar13 = 0;
      goto LAB_00496d17;
    }
    iVar13 = *(int *)((int)this + 0x20);
    uVar15 = 2;
  }
  iVar13 = FUN_004f3dd0(iVar13,uVar15);
LAB_00496d17:
  piVar4 = FUN_004f3220(*(int *)((int)this + 0x20),(uint *)((int)this + 0x5c));
  if (*(int *)((int)this + 100) == 0) {
    if (*(int *)((int)this + 0x20) != *(int *)((int)this + 0x6c)) {
      *(undefined4 *)((int)this + 0x28) = 0x2e;
    }
    if (piVar4 == (int *)0x0) {
      piVar10 = (int *)0x0;
    }
    else {
      piVar10 = piVar4 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7098,piVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (iVar13 == 0) {
      if (piVar4 == (int *)0x0) {
        piVar10 = (int *)0x0;
      }
      else {
        piVar10 = piVar4 + 0xc;
      }
      if (iVar9 == 0) {
        puVar11 = (undefined4 *)0x0;
      }
      else {
        puVar11 = (undefined4 *)(iVar9 + 0x30);
      }
      piVar4 = (int *)0x0;
      uVar15 = 0x709a;
    }
    else {
      if (piVar4 == (int *)0x0) {
        piVar4 = (int *)0x0;
      }
      else {
        piVar4 = piVar4 + 0xc;
      }
      piVar10 = (int *)(iVar13 + 0x30);
      if (iVar9 == 0) {
        puVar11 = (undefined4 *)0x0;
        uVar15 = 0x7099;
      }
      else {
        puVar11 = (undefined4 *)(iVar9 + 0x30);
        uVar15 = 0x7099;
      }
    }
    FUN_0060b9d0((void *)((int)this + 0x50),uVar15,puVar11,piVar10,piVar4,(undefined4 *)0x0);
    iVar9 = thunk_FUN_005f5060((int)this + 0x80);
    if (iVar9 == 0) {
      iVar9 = thunk_FUN_005f5060((int)this + 0x88);
      if (iVar9 == 0) {
        *(undefined2 *)((int)this + 0x2e) = 0x2b9a;
      }
      else {
        *(undefined2 *)((int)this + 0x2e) = 0x2b9c;
      }
    }
    else {
      *(undefined2 *)((int)this + 0x2e) = 0x2b9b;
    }
  }
  else {
    if (*(int *)((int)this + 0x20) != *(int *)((int)this + 0x6c)) {
      *(undefined4 *)((int)this + 0x28) = 0x2f;
    }
    if (piVar4 == (int *)0x0) {
      piVar10 = (int *)0x0;
    }
    else {
      piVar10 = piVar4 + 0xc;
    }
    FUN_0060b9d0((void *)((int)this + 0x44),0x7090,piVar10,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    uVar15 = *(uint *)((int)this + 0x6c);
    bVar14 = ((uint)piVar4[9] >> 6 & 3) != uVar15;
    if (uVar15 == 1) {
      *(undefined2 *)((int)this + 0x2e) = 0x2b98;
    }
    else {
      *(undefined2 *)((int)this + 0x2e) = 0x2b99;
    }
    if (iVar13 == 0) {
      if (bVar14) {
        if (piVar4 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar4 + 0xc;
        }
        if (iVar9 == 0) {
          puVar11 = (undefined4 *)0x0;
        }
        else {
          puVar11 = (undefined4 *)(iVar9 + 0x30);
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7094,puVar11,piVar4,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
      else {
        if (piVar4 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar4 + 0xc;
        }
        if (iVar9 == 0) {
          puVar11 = (undefined4 *)0x0;
        }
        else {
          puVar11 = (undefined4 *)(iVar9 + 0x30);
        }
        FUN_0060b9d0((void *)((int)this + 0x50),0x7093,puVar11,piVar4,(undefined4 *)0x0,
                     (undefined4 *)0x0);
      }
    }
    else if (bVar14) {
      if (piVar4 == (int *)0x0) {
        piVar4 = (int *)0x0;
      }
      else {
        piVar4 = piVar4 + 0xc;
      }
      if (iVar9 == 0) {
        puVar11 = (undefined4 *)0x0;
      }
      else {
        puVar11 = (undefined4 *)(iVar9 + 0x30);
      }
      FUN_0060b9d0((void *)((int)this + 0x50),0x7092,puVar11,(undefined4 *)(iVar13 + 0x30),piVar4,
                   (undefined4 *)0x0);
    }
    else {
      if (piVar4 == (int *)0x0) {
        piVar4 = (int *)0x0;
      }
      else {
        piVar4 = piVar4 + 0xc;
      }
      if (iVar9 == 0) {
        puVar11 = (undefined4 *)0x0;
      }
      else {
        puVar11 = (undefined4 *)(iVar9 + 0x30);
      }
      FUN_0060b9d0((void *)((int)this + 0x50),0x7091,puVar11,(undefined4 *)(iVar13 + 0x30),piVar4,
                   (undefined4 *)0x0);
    }
  }
  ExceptionList = local_c;
  return;
}

