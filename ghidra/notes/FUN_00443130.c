
void __fastcall FUN_00443130(void *param_1)

{
  undefined4 *puVar1;
  HWND hWnd;
  byte bVar2;
  bool bVar3;
  int iVar4;
  void *pvVar5;
  uint *puVar6;
  HDC pHVar7;
  HDC hdc;
  int iVar8;
  uint *puVar9;
  HGDIOBJ pvVar10;
  undefined3 extraout_var;
  int *piVar11;
  undefined1 *puVar12;
  int iVar13;
  uint uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined1 uVar18;
  uint *local_54;
  HDC local_50;
  undefined1 *local_40;
  undefined4 local_3c;
  int aiStack_38 [3];
  int aiStack_2c [4];
  int aiStack_1c [3];
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ed77;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_50 = (HDC)FUN_006037f0(7);
  FUN_006037f0(2);
  iVar8 = *(int *)(*(int *)((int)param_1 + 0x114) + 0x9c);
  local_54 = (uint *)0x0;
  iVar4 = FUN_004ece60((uint *)((int)param_1 + 0x124));
  if (iVar4 == 0) {
LAB_00443208:
    uVar14 = -(uint)(iVar8 != 1) & 4;
LAB_00443210:
    uVar14 = uVar14 + 0x2d22;
  }
  else {
    local_40 = (undefined1 *)0xf1;
    uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
    local_3c = 0xf2;
    if ((uVar14 < 0xf1) || (0xf1 < uVar14)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) goto LAB_00443208;
    uVar14 = *(uint *)(*(int *)((int)param_1 + 300) + 0x24) >> 6 & 3;
    if (uVar14 == 1) {
      uVar14 = -(uint)(iVar8 != 1) & 3;
      goto LAB_00443210;
    }
    if (uVar14 == 2) {
      uVar14 = (-(uint)(iVar8 != 1) & 3) + 0x2d23;
    }
    else {
      uVar14 = (-(uint)(iVar8 != 1) & 3) + 0x2d24;
    }
  }
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (pvVar5 == (void *)0x0) {
    puVar6 = (uint *)0x0;
  }
  else {
    puVar6 = (uint *)FUN_005fbd20(pvVar5,local_50,uVar14,2);
  }
  hWnd = *(HWND *)((int)param_1 + 0x18);
  local_4 = 0xffffffff;
  pHVar7 = GetDC(hWnd);
  hdc = CreateCompatibleDC(pHVar7);
  ReleaseDC(hWnd,pHVar7);
  pvVar5 = (void *)FUN_00618b70(0x28);
  local_4 = 1;
  if (pvVar5 == (void *)0x0) {
    puVar9 = (uint *)0x0;
  }
  else {
    puVar9 = puVar6;
    pHVar7 = hdc;
    iVar8 = FUN_005fc0f0((int *)puVar6);
    iVar4 = FUN_005fc0e0((int *)puVar6);
    puVar9 = (uint *)FUN_005fbea0(pvVar5,iVar4,iVar8,puVar9,pHVar7);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puVar6,puVar9,0,0,0,0,(undefined4 *)0x0,0);
  if (puVar6 != (uint *)0x0) {
    FUN_005fbfa0(puVar6);
    FUN_00618b60((undefined *)puVar6);
  }
  pvVar10 = (HGDIOBJ)FUN_005fc130((int)puVar9);
  pvVar10 = SelectObject(hdc,pvVar10);
  SelectObject(hdc,pvVar10);
  FUN_006075b0(param_1,(int *)puVar9,10);
  SetFocus(*(HWND *)((int)param_1 + 0x18));
  pvVar5 = (void *)FUN_00618b70(0x118);
  local_4 = 2;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uVar17 = 0x23;
    uVar16 = 0xd0;
    uVar15 = 0x67;
    puVar6 = (uint *)FUN_006073d0(param_1);
    pvVar5 = FUN_00607ea0(pvVar5,local_50,0x12,0x2f,0xd0,0xcc,param_1,puVar6,uVar15,uVar16,uVar17);
  }
  *(void **)((int)param_1 + 0x118) = pvVar5;
  local_4 = 0xffffffff;
  if (pvVar5 != (void *)0x0) {
    pvVar5 = (void *)FUN_00618b70(0x10);
    local_4 = 3;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_0060a790(pvVar5,4);
    }
    *(void **)((int)param_1 + 0x11c) = pvVar5;
    local_4 = 0xffffffff;
    *(uint *)(*(int *)((int)param_1 + 0x118) + 0x50) =
         *(uint *)(*(int *)((int)param_1 + 0x118) + 0x50) & 0xefffffff;
    *(undefined4 *)(*(int *)((int)param_1 + 0x118) + 0xf4) = 0xa0000;
    (**(code **)(**(int **)((int)param_1 + 0x118) + 0x18))(4);
    *(undefined4 *)(*(int *)((int)param_1 + 0x118) + 0xe4) = 9;
    FUN_00609950(*(void **)((int)param_1 + 0x118),0x2ffffff);
    FUN_00609290(*(void **)((int)param_1 + 0x118),*(HWND *)(*(int *)((int)param_1 + 0x114) + 0x18));
    FUN_005ffce0(*(void **)((int)param_1 + 0x118),0);
    FUN_00608280(*(void **)((int)param_1 + 0x118),*(undefined4 *)((int)param_1 + 0x11c));
    iVar8 = *(int *)((int)param_1 + 0x118);
    FUN_0060a490((void *)(iVar8 + 0x94),*(undefined4 *)((int)param_1 + 0x1c),iVar8,7,0x299a,
                 *(undefined4 *)(iVar8 + 0x3c),0,*(undefined4 *)(iVar8 + 0x3c),0,(uint *)0x0);
    FUN_005f4f10((void *)((int)param_1 + 0x6c),*(int *)((int)param_1 + 0x118));
  }
  puVar1 = (undefined4 *)((int)param_1 + 0x130);
  FUN_00601b30(puVar1,0xf,0x12);
  *puVar1 = 0xd3;
  *(undefined4 *)((int)param_1 + 0x134) = 0x12;
  *(undefined4 *)((int)param_1 + 0x15c) = 3;
  FUN_00403e90(puVar1,0x21);
  FUN_00601c90(puVar1,0xffffff,(HWND)0x0);
  FUN_00601c60(puVar1,5);
  puVar1 = (undefined4 *)((int)param_1 + 0x198);
  FUN_00601b30(puVar1,0xf2,0x89);
  *puVar1 = 0x82;
  *(undefined4 *)((int)param_1 + 0x19c) = 0x2c;
  *(undefined4 *)((int)param_1 + 0x1c4) = 3;
  FUN_00403e90(puVar1,0x11);
  FUN_00601c90(puVar1,0xffffff,(HWND)0x0);
  FUN_00601c60(puVar1,5);
  iVar8 = *(int *)((int)param_1 + 0x114);
  FUN_00606980(param_1,*(int *)(iVar8 + 0xcc),*(int *)(iVar8 + 0xd0),*(int *)(iVar8 + 0xd4),
               *(int *)(iVar8 + 0xd8));
  puVar6 = (uint *)((int)param_1 + 0x124);
  iVar8 = FUN_004ece60(puVar6);
  if (iVar8 == 0) {
    FUN_0044a210((int)param_1);
    if (*(undefined4 **)((int)param_1 + 0x11c) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)param_1 + 0x11c))(1);
    }
    pvVar5 = (void *)FUN_00618b70(0x10);
    local_4 = 4;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_0060a790(pvVar5,1);
    }
    local_4 = 0xffffffff;
    *(void **)((int)param_1 + 0x11c) = pvVar5;
    FUN_00608280(*(void **)((int)param_1 + 0x118),pvVar5);
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 5;
    if (pvVar5 == (void *)0x0) {
      local_54 = (uint *)0x0;
    }
    else {
      local_54 = (uint *)FUN_005fbd20(pvVar5,local_50,0x2d46,2);
    }
    local_4 = 0xffffffff;
    *(undefined4 *)(*(int *)((int)param_1 + 0x118) + 0xe4) = 0x24;
    *(undefined4 *)(*(int *)((int)param_1 + 0x118) + 0xe8) = 0x28;
    if (*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c) == 1) {
      FUN_0044b2d0((int)param_1);
    }
    else {
      FUN_0044bb70((int)param_1);
    }
  }
  else {
    local_40 = (undefined1 *)0xf1;
    local_3c = 0xf2;
    if ((*puVar6 >> 0x18 < 0xf1) || (0xf1 < *puVar6 >> 0x18)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      FUN_005f52c0(aiStack_38);
      local_4 = 6;
      FUN_0044a210((int)param_1);
      if (*(undefined4 **)((int)param_1 + 0x11c) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)param_1 + 0x11c))(1);
      }
      pvVar5 = (void *)FUN_00618b70(0x10);
      local_4._0_1_ = 7;
      if (pvVar5 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        pvVar5 = FUN_0060a790(pvVar5,1);
      }
      local_4._0_1_ = 6;
      *(void **)((int)param_1 + 0x11c) = pvVar5;
      FUN_00608280(*(void **)((int)param_1 + 0x118),pvVar5);
      pvVar5 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 8;
      if (pvVar5 == (void *)0x0) {
        local_54 = (uint *)0x0;
      }
      else {
        local_54 = (uint *)FUN_005fbd20(pvVar5,local_50,0x2d46,2);
      }
      local_4._0_1_ = 6;
      iVar8 = *(int *)(*(int *)((int)param_1 + 0x114) + 0x9c);
      piVar11 = FUN_004f32c0(iVar8);
      local_40 = (undefined1 *)0x20;
      local_3c = 0x22;
      local_4._0_1_ = 9;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f33d0(iVar8);
      local_40 = (undefined1 *)0x2a;
      local_3c = 0x2b;
      local_4._0_1_ = 10;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f34b0(iVar8);
      local_40 = (undefined1 *)0x18;
      local_3c = 0x1c;
      local_4._0_1_ = 0xb;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3440(iVar8);
      local_40 = (undefined1 *)0x25;
      local_3c = 0x26;
      local_4._0_1_ = 0xc;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3520(iVar8);
      local_40 = (undefined1 *)0x1c;
      local_3c = 0x20;
      local_4._0_1_ = 0xd;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3740(iVar8);
      local_40 = (undefined1 *)0x22;
      local_3c = 0x23;
      local_4._0_1_ = 0xe;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f37b0(iVar8);
      local_40 = (undefined1 *)0x2c;
      local_3c = 0x2d;
      local_4._0_1_ = 0xf;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3820(iVar8);
      local_40 = (undefined1 *)0x24;
      local_3c = 0x25;
      local_4._0_1_ = 0x10;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3890(iVar8);
      local_40 = (undefined1 *)0x2d;
      local_3c = 0x2e;
      local_4._0_1_ = 0x11;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3a10(iVar8);
      local_40 = (undefined1 *)0x28;
      local_3c = 0x29;
      local_4._0_1_ = 0x12;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3a80(iVar8);
      local_40 = (undefined1 *)0x3c;
      local_3c = 0x40;
      local_4._0_1_ = 0x13;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3b90(iVar8);
      local_40 = (undefined1 *)0x14;
      local_3c = 0x18;
      local_4._0_1_ = 0x14;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3c00(iVar8);
      local_40 = (undefined1 *)0x29;
      local_3c = 0x2a;
      local_4._0_1_ = 0x15;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3c70(iVar8);
      local_40 = (undefined1 *)0x10;
      local_3c = 0x14;
      local_4._0_1_ = 0x16;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4._0_1_ = 6;
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      piVar11 = FUN_004f3ce0(iVar8);
      local_40 = (undefined1 *)0x23;
      local_3c = 0x24;
      local_4._0_1_ = 0x17;
      FUN_0044a7a0(aiStack_38,piVar11,(uint *)&local_40);
      local_4 = CONCAT31(local_4._1_3_,6);
      FUN_00619730();
      FUN_0044aa20(param_1,(int)aiStack_38);
      iVar8 = (**(code **)(**(int **)((int)param_1 + 0x11c) + 8))();
      if ((iVar8 != 0) && (*(int *)(iVar8 + 0x20) != 0)) {
        *(undefined1 *)(*(int *)((int)param_1 + 0x118) + 0xd4) =
             *(undefined1 *)(*(int *)(iVar8 + 0x20) + 8);
      }
      local_4 = 0xffffffff;
      FUN_005f5300(aiStack_38);
    }
    else {
      local_40 = (undefined1 *)0x50;
      local_3c = 0x80;
      if ((*puVar6 >> 0x18 < 0x50) || (0x7f < *puVar6 >> 0x18)) {
        bVar3 = false;
      }
      else {
        bVar3 = true;
      }
      FUN_00619730();
      if (bVar3) {
        piVar11 = FUN_004f2d10(*(int *)(*(int *)((int)param_1 + 0x114) + 0x9c),puVar6);
        if (piVar11 != (int *)0x0) {
          if (((byte)piVar11[9] & 0xc0) == 0x40) {
            uVar14 = *(uint *)(piVar11[0xb] + 0x30) & 0xfff;
          }
          else {
            uVar14 = (*(uint *)(piVar11[0xb] + 0x30) & 0xfff) + 0x1000;
          }
          pvVar5 = (void *)FUN_00618b70(0x28);
          local_4 = 0x18;
          if (pvVar5 == (void *)0x0) {
LAB_00443daa:
            local_54 = (uint *)0x0;
          }
          else {
            local_54 = (uint *)FUN_005fbd20(pvVar5,*(undefined4 *)((int)param_1 + 0x1d4),uVar14,2);
          }
LAB_00443dac:
          local_4 = 0xffffffff;
        }
      }
      else {
        local_40 = (undefined1 *)0xa0;
        local_3c = 0xb0;
        if ((*puVar6 >> 0x18 < 0xa0) || (0xaf < *puVar6 >> 0x18)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        if (bVar3) {
          local_40 = (undefined1 *)0xa0;
          local_3c = 0xa2;
          if ((*puVar6 >> 0x18 < 0xa0) || (0xa1 < *puVar6 >> 0x18)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          FUN_00619730();
          if (bVar3) {
            uVar14 = 0x107;
          }
          else {
            local_40 = (undefined1 *)0xa2;
            local_3c = 0xa4;
            if ((*puVar6 >> 0x18 < 0xa2) || (0xa3 < *puVar6 >> 0x18)) {
              bVar2 = 0;
            }
            else {
              bVar2 = 1;
            }
            FUN_00619730();
            uVar14 = (-(uint)bVar2 & 0xfffffffe) + 0x108;
          }
          pvVar5 = (void *)FUN_00618b70(0x28);
          local_4 = 0x19;
          if (pvVar5 == (void *)0x0) goto LAB_00443daa;
          local_54 = (uint *)FUN_005fbd20(pvVar5,*(undefined4 *)((int)param_1 + 0x1d4),uVar14,2);
          goto LAB_00443dac;
        }
        local_54 = FUN_0042c3b0(*(undefined4 *)((int)param_1 + 0x1d4),puVar6,
                                *(int **)((int)param_1 + 300),1,1);
      }
      pvVar5 = (void *)FUN_00618b70(0x68);
      local_4 = 0x1a;
      if (pvVar5 == (void *)0x0) {
        local_50 = (HDC)0x0;
      }
      else {
        local_50 = (HDC)FUN_0060ba00(pvVar5,1);
      }
      local_4 = 0xffffffff;
      if (local_50 != (HDC)0x0) {
        FUN_0060a790(aiStack_1c,1);
        local_4 = 0x1b;
        FUN_0060a790(aiStack_2c,1);
        local_4._0_1_ = 0x1c;
        local_40 = (undefined1 *)0x8;
        local_3c = 0x10;
        if ((*puVar6 >> 0x18 < 8) || (0xf < *puVar6 >> 0x18)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        if (bVar3) {
          FUN_00449200(param_1,aiStack_1c,aiStack_2c);
        }
        else {
          local_40 = (undefined1 *)0x14;
          local_3c = 0x1c;
          if ((*puVar6 >> 0x18 < 0x14) || (0x1b < *puVar6 >> 0x18)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          FUN_00619730();
          if (bVar3) {
            FUN_00446fd0(param_1,aiStack_1c,aiStack_2c);
          }
          else {
            local_40 = (undefined1 *)0x30;
            local_3c = 0x3c;
            if ((*puVar6 >> 0x18 < 0x30) || (0x3b < *puVar6 >> 0x18)) {
              bVar3 = false;
            }
            else {
              bVar3 = true;
            }
            FUN_00619730();
            if (bVar3) {
              FUN_004486f0(param_1,aiStack_1c,aiStack_2c);
            }
            else {
              uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
              local_4 = CONCAT31(local_4._1_3_,0x1d);
              if ((uVar14 < 0x22) || (0x27 < uVar14)) {
                local_40 = &DAT_00000020;
                local_3c = 0x22;
                if ((uVar14 < 0x20) || (0x21 < uVar14)) {
                  bVar3 = false;
                }
                else {
                  bVar3 = true;
                }
                FUN_00619730();
                if (bVar3) goto LAB_00443f86;
                bVar3 = false;
              }
              else {
LAB_00443f86:
                bVar3 = true;
              }
              local_4._0_1_ = 0x1c;
              FUN_00619730();
              if (bVar3) {
                FUN_00444e20(param_1,aiStack_1c,aiStack_2c);
              }
              else {
                local_40 = (undefined1 *)0x1c;
                uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                local_3c = 0x20;
                if ((uVar14 < 0x1c) || (0x1f < uVar14)) {
                  bVar3 = false;
                }
                else {
                  bVar3 = true;
                }
                FUN_00619730();
                if (bVar3) {
                  FUN_004444c0(param_1,aiStack_1c,aiStack_2c);
                }
                else {
                  local_40 = (undefined1 *)0x10;
                  uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                  local_3c = 0x14;
                  if ((uVar14 < 0x10) || (0x13 < uVar14)) {
                    bVar3 = false;
                  }
                  else {
                    bVar3 = true;
                  }
                  FUN_00619730();
                  if (bVar3) {
                    FUN_00445280(param_1,aiStack_1c,aiStack_2c);
                  }
                  else {
                    local_40 = (undefined1 *)0x28;
                    uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                    local_3c = 0x30;
                    if ((uVar14 < 0x28) || (0x2f < uVar14)) {
                      bVar3 = false;
                    }
                    else {
                      bVar3 = true;
                    }
                    FUN_00619730();
                    if (bVar3) {
                      FUN_00446c50(param_1,aiStack_1c,aiStack_2c);
                    }
                    else {
                      local_40 = (undefined1 *)0x90;
                      uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                      local_3c = 0x98;
                      if ((uVar14 < 0x90) || (0x97 < uVar14)) {
                        bVar3 = false;
                      }
                      else {
                        bVar3 = true;
                      }
                      FUN_00619730();
                      if (bVar3) {
                        FUN_00446250(param_1,aiStack_1c,aiStack_2c);
                      }
                      else {
                        local_40 = (undefined1 *)0x50;
                        uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                        local_3c = 0x80;
                        if ((uVar14 < 0x50) || (0x7f < uVar14)) {
                          bVar3 = false;
                        }
                        else {
                          bVar3 = true;
                        }
                        FUN_00619730();
                        if (bVar3) {
                          iVar8 = FUN_00520b70(*(int *)((int)param_1 + 300));
                          if (iVar8 == 0) {
                            FUN_00445c60(param_1,aiStack_1c,aiStack_2c);
                          }
                        }
                        else {
                          local_40 = (undefined1 *)0x3c;
                          uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                          local_3c = 0x40;
                          if ((uVar14 < 0x3c) || (0x3f < uVar14)) {
                            bVar3 = false;
                          }
                          else {
                            bVar3 = true;
                          }
                          FUN_00619730();
                          if (bVar3) {
                            FUN_00445780(param_1,aiStack_1c,aiStack_2c);
                          }
                          else {
                            aiStack_38[0] = 0xa0;
                            uVar14 = *(uint *)((int)param_1 + 0x124) >> 0x18;
                            aiStack_38[1] = 0xb0;
                            if ((uVar14 < 0xa0) || (0xaf < uVar14)) {
                              bVar3 = false;
                            }
                            else {
                              bVar3 = true;
                            }
                            FUN_00619730();
                            if (!bVar3) {
                              local_4 = CONCAT31(local_4._1_3_,0x1b);
                              FUN_0060a810(aiStack_2c);
                              local_4 = 0xffffffff;
                              FUN_0060a810(aiStack_1c);
                              ExceptionList = pvStack_c;
                              return;
                            }
                            FUN_0044acd0(param_1,aiStack_1c,aiStack_2c);
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
        iVar8 = FUN_00449e00(hdc,aiStack_1c,aiStack_2c);
        if (iVar8 != 0) {
          puVar12 = (undefined1 *)FUN_00618b70(0x28);
          local_4._0_1_ = 0x1e;
          local_40 = puVar12;
          if (puVar12 == (undefined1 *)0x0) {
            puVar6 = (uint *)0x0;
          }
          else {
            pHVar7 = hdc;
            puVar6 = (uint *)FUN_006073d0(param_1);
            puVar6 = (uint *)FUN_005fbea0(puVar12,0xcf,iVar8,puVar6,pHVar7);
          }
          uVar18 = 0;
          local_4._0_1_ = 0x1c;
          iVar4 = FUN_005fc0f0((int *)puVar6);
          uVar14 = FUN_005fc0e0((int *)puVar6);
          FUN_005fcfe0(puVar6,0,0,uVar14,iVar4,uVar18);
          pvVar10 = (HGDIOBJ)FUN_005fc130((int)puVar6);
          pvVar10 = SelectObject(hdc,pvVar10);
          FUN_0044a050(hdc,aiStack_1c,aiStack_2c);
          SelectObject(hdc,pvVar10);
          puVar12 = (undefined1 *)FUN_00618b70(0x28);
          local_4._0_1_ = 0x1f;
          local_40 = puVar12;
          if (puVar12 == (undefined1 *)0x0) {
            puVar9 = (uint *)0x0;
          }
          else {
            puVar9 = puVar6;
            iVar4 = FUN_005fc0f0((int *)puVar6);
            iVar13 = FUN_005fc0e0((int *)puVar6);
            puVar9 = (uint *)FUN_005fbea0(puVar12,iVar13,iVar4,puVar9,hdc);
          }
          local_4._0_1_ = 0x1c;
          FUN_005fcc30(puVar6,puVar9,0,0,0,0,(undefined4 *)0x0,0);
          FUN_0060be60(local_50,(int)puVar6,(int)puVar9);
          FUN_006082c0(*(void **)((int)param_1 + 0x118),0xcf,iVar8);
        }
        FUN_005f59f0(*(void **)((int)param_1 + 0x11c),(int)local_50);
        local_4 = CONCAT31(local_4._1_3_,0x1b);
        FUN_0060a810(aiStack_2c);
        local_4 = 0xffffffff;
        FUN_0060a810(aiStack_1c);
      }
    }
  }
  if (local_54 != (uint *)0x0) {
    bVar3 = FUN_005fc040((int *)local_54);
    if (CONCAT31(extraout_var,bVar3) == 0) {
      if (local_54 == (uint *)0x0) goto LAB_00443668;
    }
    else {
      iVar8 = FUN_005fc0e0((int *)local_54);
      iVar4 = 0x133 - iVar8 / 2;
      iVar8 = FUN_005fc0f0((int *)local_54);
      iVar8 = 0x40 - iVar8 / 2;
      puVar6 = (uint *)FUN_006073d0(param_1);
      FUN_005fd0f0(local_54,puVar6,iVar4,iVar8);
    }
    FUN_005fbfa0(local_54);
    FUN_00618b60((undefined *)local_54);
  }
LAB_00443668:
  FUN_0060a280(*(void **)((int)param_1 + 0x118));
  (**(code **)(**(int **)((int)param_1 + 0x118) + 4))(5);
  DeleteDC(local_50);
  ExceptionList = pvStack_10;
  return;
}

