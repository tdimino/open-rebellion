
void __thiscall FUN_00462be0(void *this,uint param_1)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  bool bVar7;
  bool bVar8;
  bool bVar9;
  uint uVar10;
  int *piVar11;
  uint *puVar12;
  void *pvVar13;
  char *pcVar14;
  int iVar15;
  void *pvVar16;
  undefined4 local_44;
  uint local_40;
  uint local_3c;
  uint local_38;
  uint local_34;
  void *local_30;
  uint local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  RECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006314d4;
  local_c = ExceptionList;
  bVar8 = false;
  bVar6 = false;
  bVar1 = false;
  bVar9 = false;
  bVar7 = false;
  bVar5 = false;
  bVar4 = false;
  bVar3 = false;
  bVar2 = false;
  iVar15 = *(int *)(*(int *)((int)this + 0x158) + 0x9c);
  if (*(uint *)((int)this + 0x160) != param_1) {
    ExceptionList = &local_c;
    FUN_004ece30(&local_44);
    local_4._0_1_ = 0;
    local_4._1_3_ = 0;
    FUN_005f5b20(*(void **)((int)this + 0x168));
    if (*(int *)((int)this + 0x148) == 1) {
      piVar11 = FUN_004f3630(iVar15);
      while (piVar11 != (int *)0x0) {
        pvVar16 = (void *)0x0;
        if ((*(byte *)(piVar11 + 0x14) & 0x40) != 0) {
          puVar12 = FUN_004025b0((void *)piVar11[7],&local_40);
          local_24 = 0xf2;
          local_20 = 0xf3;
          local_4._0_1_ = 1;
          if ((*puVar12 >> 0x18 < 0xf2) || (0xf2 < *puVar12 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 0;
          FUN_00619730();
          if (!bVar1) {
            if (param_1 == 1) {
              pvVar16 = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 2;
              local_30 = pvVar16;
              if (pvVar16 == (void *)0x0) {
                pvVar16 = (void *)0x0;
              }
              else {
                puVar12 = FUN_004025b0(piVar11,&local_34);
                bVar4 = true;
                local_4 = CONCAT31(local_4._1_3_,3);
                pvVar16 = FUN_004421d0(pvVar16,puVar12);
              }
              local_4 = 2;
              if (bVar4) {
                bVar4 = false;
LAB_00462e20:
                FUN_00619730();
              }
LAB_00462e25:
              local_4._0_1_ = 0;
            }
            else if (param_1 == 2) {
              if (((byte)piVar11[9] & 0xc0) == 0x40) {
                pvVar16 = (void *)FUN_00618b70(0x74);
                local_4._0_1_ = 4;
                local_30 = pvVar16;
                if (pvVar16 == (void *)0x0) {
                  pvVar16 = (void *)0x0;
                }
                else {
                  puVar12 = FUN_004025b0(piVar11,&local_38);
                  bVar3 = true;
                  local_4 = CONCAT31(local_4._1_3_,5);
                  pvVar16 = FUN_004421d0(pvVar16,puVar12);
                }
                local_4 = 4;
                if (bVar3) {
                  bVar3 = false;
                  goto LAB_00462e20;
                }
                goto LAB_00462e25;
              }
            }
            else if ((param_1 == 3) && (((byte)piVar11[9] & 0xc0) == 0x80)) {
              pvVar16 = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 6;
              local_30 = pvVar16;
              if (pvVar16 == (void *)0x0) {
                pvVar16 = (void *)0x0;
              }
              else {
                puVar12 = FUN_004025b0(piVar11,&local_3c);
                bVar2 = true;
                local_4 = CONCAT31(local_4._1_3_,7);
                pvVar16 = FUN_004421d0(pvVar16,puVar12);
              }
              local_4 = 6;
              if (bVar2) {
                bVar2 = false;
                goto LAB_00462e20;
              }
              goto LAB_00462e25;
            }
            if (pvVar16 != (void *)0x0) {
              pvVar13 = FUN_004f6270(piVar11,&local_1c);
              local_4._0_1_ = 8;
              pcVar14 = (char *)FUN_00583c40((int)pvVar13);
              FUN_005f35e0((void *)((int)pvVar16 + 0x14),pcVar14);
              local_4._0_1_ = 0;
              FUN_005f2ff0(&local_1c.left);
              FUN_005f59f0(*(void **)((int)this + 0x168),(int)pvVar16);
            }
          }
        }
        local_2c = 8;
        local_28 = 0x10;
        local_4._0_1_ = 9;
        piVar11 = (int *)FUN_004f6010(piVar11,&local_2c,1);
        local_4._0_1_ = 0;
        FUN_00619730();
      }
    }
    else {
      piVar11 = FUN_004f2db0(iVar15);
      while (piVar11 != (int *)0x0) {
        pvVar16 = (void *)0x0;
        if ((*(byte *)(piVar11 + 0x14) & 0x40) == 0) {
LAB_00462f36:
          bVar2 = false;
        }
        else {
          puVar12 = FUN_004025b0(*(void **)(piVar11[7] + 0x1c),&local_34);
          local_24 = 0xf2;
          local_20 = 0xf3;
          local_4._0_1_ = 10;
          if ((*puVar12 >> 0x18 < 0xf2) || (0xf2 < *puVar12 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          local_4._0_1_ = 0;
          FUN_00619730();
          bVar1 = bVar5;
          bVar6 = bVar7;
          bVar8 = bVar9;
          if (bVar2) goto LAB_00462f36;
          bVar2 = true;
        }
        if (bVar2) {
          if (param_1 == 1) {
            pvVar16 = (void *)FUN_00618b70(0x74);
            local_4._0_1_ = 0xb;
            local_30 = pvVar16;
            if (pvVar16 == (void *)0x0) {
              pvVar16 = (void *)0x0;
            }
            else {
              puVar12 = FUN_004025b0(piVar11,&local_40);
              bVar8 = true;
              local_4 = CONCAT31(local_4._1_3_,0xc);
              pvVar16 = FUN_004421d0(pvVar16,puVar12);
              bVar5 = bVar1;
              bVar7 = bVar6;
              bVar9 = bVar8;
            }
            local_4 = 0xb;
            if (bVar8) {
              bVar8 = false;
LAB_00463087:
              FUN_00619730();
              bVar5 = bVar1;
              bVar7 = bVar6;
              bVar9 = bVar8;
            }
LAB_0046308c:
            local_4._0_1_ = 0;
          }
          else if (param_1 == 2) {
            if (((byte)piVar11[9] & 0xc0) == 0x40) {
              pvVar16 = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 0xd;
              local_30 = pvVar16;
              if (pvVar16 == (void *)0x0) {
                pvVar16 = (void *)0x0;
              }
              else {
                puVar12 = FUN_004025b0(piVar11,&local_3c);
                bVar6 = true;
                local_4 = CONCAT31(local_4._1_3_,0xe);
                pvVar16 = FUN_004421d0(pvVar16,puVar12);
                bVar5 = bVar1;
                bVar7 = bVar6;
                bVar9 = bVar8;
              }
              local_4 = 0xd;
              if (bVar6) {
                bVar6 = false;
                goto LAB_00463087;
              }
              goto LAB_0046308c;
            }
          }
          else if ((param_1 == 3) && (((byte)piVar11[9] & 0xc0) == 0x80)) {
            pvVar16 = (void *)FUN_00618b70(0x74);
            local_4._0_1_ = 0xf;
            local_30 = pvVar16;
            if (pvVar16 == (void *)0x0) {
              pvVar16 = (void *)0x0;
            }
            else {
              puVar12 = FUN_004025b0(piVar11,&local_38);
              bVar1 = true;
              local_4 = CONCAT31(local_4._1_3_,0x10);
              pvVar16 = FUN_004421d0(pvVar16,puVar12);
              bVar5 = bVar1;
              bVar7 = bVar6;
              bVar9 = bVar8;
            }
            local_4 = 0xf;
            if (bVar1) {
              bVar1 = false;
              goto LAB_00463087;
            }
            goto LAB_0046308c;
          }
          if (pvVar16 != (void *)0x0) {
            pvVar13 = FUN_004f6270(piVar11,&local_1c);
            local_4._0_1_ = 0x11;
            pcVar14 = (char *)FUN_00583c40((int)pvVar13);
            FUN_005f35e0((void *)((int)pvVar16 + 0x14),pcVar14);
            local_4._0_1_ = 0;
            FUN_005f2ff0(&local_1c.left);
            FUN_005f59f0(*(void **)((int)this + 0x168),(int)pvVar16);
          }
        }
        local_2c = 0x14;
        local_28 = 0x1c;
        local_4._0_1_ = 0x12;
        piVar11 = (int *)FUN_004f6010(piVar11,&local_2c,1);
        local_4._0_1_ = 0;
        FUN_00619730();
      }
    }
    uVar10 = param_1;
    *(uint *)((int)this + 0x160) = param_1;
    FUN_004f26d0((void *)((int)this + 0x16c),&local_44);
    FUN_0060a280(*(void **)((int)this + 0x154));
    if (*(int *)((int)this + 0x148) == 1) {
      pvVar16 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),1);
      param_1 = CONCAT22(DAT_0065d424,0x1882);
      FUN_00600c40(this,pvVar16,&param_1);
      pvVar16 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),2);
      param_1 = CONCAT22(DAT_0065d424,0x1883);
      FUN_00600c40(this,pvVar16,&param_1);
      pvVar16 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),3);
      param_1 = CONCAT22(DAT_0065d424,0x1884);
    }
    else {
      pvVar16 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),1);
      param_1 = CONCAT22(DAT_0065d424,0x1887);
      FUN_00600c40(this,pvVar16,&param_1);
      pvVar16 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),2);
      param_1 = CONCAT22(DAT_0065d424,0x1888);
      FUN_00600c40(this,pvVar16,&param_1);
      pvVar16 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),3);
      param_1 = CONCAT22(DAT_0065d424,0x1889);
    }
    FUN_00600c40(this,pvVar16,&param_1);
    iVar15 = FUN_00604500((void *)(*(int *)((int)this + 0x14c) + 0x6c),uVar10);
    FUN_00601aa0(*(void **)((int)this + 0x15c),*(char **)(iVar15 + 0x84));
    iVar15 = *(int *)((int)this + 0x15c);
    local_1c.left = *(LONG *)(iVar15 + 0x14);
    local_1c.top = *(LONG *)(iVar15 + 0x18);
    local_1c.right = *(LONG *)(iVar15 + 0x1c);
    local_1c.bottom = *(LONG *)(iVar15 + 0x20);
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return;
}

