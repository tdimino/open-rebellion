
int * __thiscall FUN_004568a0(void *this,int param_1)

{
  bool bVar1;
  int *piVar2;
  int *this_00;
  uint *puVar3;
  void *pvVar4;
  uint *puVar5;
  int iVar6;
  undefined4 *puVar7;
  void *this_01;
  void *pvVar8;
  int iVar9;
  undefined1 uStack_6c;
  uint *puStack_64;
  uint *puStack_60;
  void *pvStack_5c;
  void *pvStack_58;
  int *local_54;
  void *pvStack_50;
  void *local_4c;
  undefined4 local_48 [3];
  tagRECT tStack_3c;
  undefined4 auStack_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063042a;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_4c = (void *)FUN_006037f0(7);
  piVar2 = FUN_004a25c0((int *)this);
  if (piVar2 != (int *)0x0) {
    if (*(int *)((int)this + 0x160) == param_1) {
      piVar2 = (int *)0x0;
    }
    else {
      local_54 = piVar2;
      FUN_005f5060((int)this + 0x6c);
      if (param_1 == 0x67) {
        FUN_00458600(this,0);
        (**(code **)(**(int **)((int)this + 0x174) + 8))();
      }
      else {
        FUN_005f2f50(local_48);
        local_4 = 0;
        FUN_00458600(this,0);
        this_00 = (int *)(**(code **)(**(int **)((int)this + 0x178) + 8))();
        if (this_00 != (int *)0x0) {
          do {
            puVar3 = (uint *)FUN_0042d170(this_00,&pvStack_58);
            local_4._0_1_ = 1;
            FUN_004f58e0((void *)((int)this + 0x114),puVar3);
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
            this_00 = (int *)(**(code **)(*this_00 + 0xc))();
          } while (this_00 != (int *)0x0);
        }
        FUN_005f5b20(*(void **)((int)this + 0x178));
        bVar1 = false;
        switch(param_1) {
        case 0x68:
          FUN_0052cb30(auStack_2c,piVar2,3,*(undefined4 *)((int)this + 0x148));
          local_4 = CONCAT31(local_4._1_3_,0x11);
          FUN_00513120((int)auStack_2c);
          while (iStack_10 != 0) {
            pvVar8 = (void *)FUN_0052bed0((int)auStack_2c);
            if ((*(byte *)((int)pvVar8 + 0x50) & 8) == 0) {
              FUN_004025b0(pvVar8,(uint *)&pvStack_5c);
              local_4._0_1_ = 0x12;
              local_4c = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 0x13;
              if (local_4c == (void *)0x0) {
                pvStack_58 = (void *)0x0;
              }
              else {
                pvStack_58 = FUN_004421d0(local_4c,(uint *)&pvStack_5c);
              }
              local_4._0_1_ = 0x12;
              puVar3 = (uint *)FUN_00458fe0(pvVar8);
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x14;
              if (local_4c == (void *)0x0) {
                puVar5 = (uint *)0x0;
              }
              else {
                puVar5 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar3);
              }
              local_4._0_1_ = 0x12;
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x15;
              if (local_4c == (void *)0x0) {
                puStack_60 = (uint *)0x0;
              }
              else {
                puStack_60 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar5);
              }
              uStack_6c = (undefined1)puVar3[2];
              local_4 = CONCAT31(local_4._1_3_,0x12);
              FUN_005fcfe0(puVar5,0,0,0x45,0x28,uStack_6c);
              FUN_005fcfe0(puStack_60,0,0,0x45,0x28,uStack_6c);
              bVar1 = true;
              iVar6 = FUN_005fc0e0((int *)puVar3);
              iVar9 = (0x45 - iVar6) / 2;
              iVar6 = FUN_005fc0f0((int *)puVar3);
              iVar6 = (0x28 - iVar6) / 2;
              FUN_005fcc30(puVar3,puVar5,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fcc30(puVar3,puStack_60,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fd0f0(*(void **)((int)this + 0x15c),puStack_60,iVar9,iVar6);
              FUN_0060be60(pvStack_58,(int)puVar5,(int)puStack_60);
              if (puVar3 != (uint *)0x0) {
                FUN_005fbfa0(puVar3);
                FUN_00618b60((undefined *)puVar3);
              }
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x16;
              if (local_4c == (void *)0x0) {
                puVar7 = (undefined4 *)0x0;
              }
              else {
                puVar7 = (undefined4 *)FUN_004acd80(local_4c,&pvStack_5c,0);
              }
              local_4._0_1_ = 0x12;
              FUN_004acba0((void *)((int)this + 0x114),puVar7,0);
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvStack_58);
              local_4 = CONCAT31(local_4._1_3_,0x11);
              FUN_00619730();
            }
            FUN_005130d0((int)auStack_2c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052cbb0(auStack_2c);
          break;
        case 0x69:
          FUN_0052c7c0(auStack_2c,piVar2,3,*(undefined4 *)((int)this + 0x148));
          local_4 = CONCAT31(local_4._1_3_,0x17);
          FUN_00513120((int)auStack_2c);
          while (iStack_10 != 0) {
            pvVar8 = (void *)FUN_0052bed0((int)auStack_2c);
            if ((*(byte *)((int)pvVar8 + 0x50) & 8) == 0) {
              FUN_004025b0(pvVar8,(uint *)&pvStack_5c);
              local_4._0_1_ = 0x18;
              local_4c = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 0x19;
              if (local_4c == (void *)0x0) {
                pvStack_58 = (void *)0x0;
              }
              else {
                pvStack_58 = FUN_004421d0(local_4c,(uint *)&pvStack_5c);
              }
              local_4._0_1_ = 0x18;
              puVar3 = (uint *)FUN_00458fe0(pvVar8);
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x1a;
              if (local_4c == (void *)0x0) {
                puVar5 = (uint *)0x0;
              }
              else {
                puVar5 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar3);
              }
              local_4._0_1_ = 0x18;
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x1b;
              if (local_4c == (void *)0x0) {
                puStack_60 = (uint *)0x0;
              }
              else {
                puStack_60 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar5);
              }
              uStack_6c = (undefined1)puVar3[2];
              local_4 = CONCAT31(local_4._1_3_,0x18);
              FUN_005fcfe0(puVar5,0,0,0x45,0x28,uStack_6c);
              FUN_005fcfe0(puStack_60,0,0,0x45,0x28,uStack_6c);
              bVar1 = true;
              iVar6 = FUN_005fc0e0((int *)puVar3);
              iVar9 = (0x45 - iVar6) / 2;
              iVar6 = FUN_005fc0f0((int *)puVar3);
              iVar6 = (0x28 - iVar6) / 2;
              FUN_005fcc30(puVar3,puVar5,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fcc30(puVar3,puStack_60,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fd0f0(*(void **)((int)this + 0x15c),puStack_60,iVar9,iVar6);
              FUN_0060be60(pvStack_58,(int)puVar5,(int)puStack_60);
              if (puVar3 != (uint *)0x0) {
                FUN_005fbfa0(puVar3);
                FUN_00618b60((undefined *)puVar3);
              }
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x1c;
              if (local_4c == (void *)0x0) {
                puVar7 = (undefined4 *)0x0;
              }
              else {
                puVar7 = (undefined4 *)FUN_004acd80(local_4c,&pvStack_5c,0);
              }
              local_4._0_1_ = 0x18;
              FUN_004acba0((void *)((int)this + 0x114),puVar7,0);
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvStack_58);
              local_4 = CONCAT31(local_4._1_3_,0x17);
              FUN_00619730();
            }
            FUN_005130d0((int)auStack_2c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052c840(auStack_2c);
          break;
        case 0x6a:
          FUN_0052c4a0(auStack_2c,piVar2,3,*(undefined4 *)((int)this + 0x148));
          local_4 = CONCAT31(local_4._1_3_,0x1d);
          FUN_00513120((int)auStack_2c);
          while (iStack_10 != 0) {
            pvVar8 = (void *)FUN_0052bed0((int)auStack_2c);
            if ((*(byte *)((int)pvVar8 + 0x50) & 8) == 0) {
              FUN_004025b0(pvVar8,(uint *)&pvStack_5c);
              local_4._0_1_ = 0x1e;
              local_4c = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 0x1f;
              if (local_4c == (void *)0x0) {
                pvStack_58 = (void *)0x0;
              }
              else {
                pvStack_58 = FUN_004421d0(local_4c,(uint *)&pvStack_5c);
              }
              local_4._0_1_ = 0x1e;
              puVar3 = (uint *)FUN_00458fe0(pvVar8);
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x20;
              if (local_4c == (void *)0x0) {
                puVar5 = (uint *)0x0;
              }
              else {
                puVar5 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar3);
              }
              local_4._0_1_ = 0x1e;
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x21;
              if (local_4c == (void *)0x0) {
                puStack_60 = (uint *)0x0;
              }
              else {
                puStack_60 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar5);
              }
              uStack_6c = (undefined1)puVar3[2];
              local_4 = CONCAT31(local_4._1_3_,0x1e);
              FUN_005fcfe0(puVar5,0,0,0x45,0x28,uStack_6c);
              FUN_005fcfe0(puStack_60,0,0,0x45,0x28,uStack_6c);
              bVar1 = true;
              iVar6 = FUN_005fc0e0((int *)puVar3);
              iVar9 = (0x45 - iVar6) / 2;
              iVar6 = FUN_005fc0f0((int *)puVar3);
              iVar6 = (0x28 - iVar6) / 2;
              FUN_005fcc30(puVar3,puVar5,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fcc30(puVar3,puStack_60,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fd0f0(*(void **)((int)this + 0x15c),puStack_60,iVar9,iVar6);
              FUN_0060be60(pvStack_58,(int)puVar5,(int)puStack_60);
              if (puVar3 != (uint *)0x0) {
                FUN_005fbfa0(puVar3);
                FUN_00618b60((undefined *)puVar3);
              }
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x22;
              if (local_4c == (void *)0x0) {
                puVar7 = (undefined4 *)0x0;
              }
              else {
                puVar7 = (undefined4 *)FUN_004acd80(local_4c,&pvStack_5c,0);
              }
              local_4._0_1_ = 0x1e;
              FUN_004acba0((void *)((int)this + 0x114),puVar7,0);
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvStack_58);
              local_4 = CONCAT31(local_4._1_3_,0x1d);
              FUN_00619730();
            }
            FUN_005130d0((int)auStack_2c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052c520(auStack_2c);
          break;
        case 0x6b:
          FUN_0052d270(auStack_2c,piVar2,3,*(undefined4 *)((int)this + 0x148));
          local_4 = CONCAT31(local_4._1_3_,0xb);
          FUN_00513120((int)auStack_2c);
          while (iStack_10 != 0) {
            pvVar8 = (void *)FUN_0052bed0((int)auStack_2c);
            if ((*(byte *)((int)pvVar8 + 0x50) & 8) == 0) {
              FUN_004025b0(pvVar8,(uint *)&pvStack_5c);
              local_4._0_1_ = 0xc;
              local_4c = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 0xd;
              if (local_4c == (void *)0x0) {
                pvStack_58 = (void *)0x0;
              }
              else {
                pvStack_58 = FUN_004421d0(local_4c,(uint *)&pvStack_5c);
              }
              local_4._0_1_ = 0xc;
              puVar3 = (uint *)FUN_00458fe0(pvVar8);
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0xe;
              if (local_4c == (void *)0x0) {
                puVar5 = (uint *)0x0;
              }
              else {
                puVar5 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar3);
              }
              local_4._0_1_ = 0xc;
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0xf;
              if (local_4c == (void *)0x0) {
                puStack_60 = (uint *)0x0;
              }
              else {
                puStack_60 = (uint *)FUN_005fbda0(local_4c,0x45,0x28,puVar5);
              }
              uStack_6c = (undefined1)puVar3[2];
              local_4 = CONCAT31(local_4._1_3_,0xc);
              FUN_005fcfe0(puVar5,0,0,0x45,0x28,uStack_6c);
              FUN_005fcfe0(puStack_60,0,0,0x45,0x28,uStack_6c);
              bVar1 = true;
              iVar6 = FUN_005fc0e0((int *)puVar3);
              iVar9 = (0x45 - iVar6) / 2;
              iVar6 = FUN_005fc0f0((int *)puVar3);
              iVar6 = (0x28 - iVar6) / 2;
              FUN_005fcc30(puVar3,puVar5,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fcc30(puVar3,puStack_60,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fd0f0(*(void **)((int)this + 0x15c),puStack_60,iVar9,iVar6);
              FUN_0060be60(pvStack_58,(int)puVar5,(int)puStack_60);
              if (puVar3 != (uint *)0x0) {
                FUN_005fbfa0(puVar3);
                FUN_00618b60((undefined *)puVar3);
              }
              local_4c = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 0x10;
              if (local_4c == (void *)0x0) {
                puVar7 = (undefined4 *)0x0;
              }
              else {
                puVar7 = (undefined4 *)FUN_004acd80(local_4c,&pvStack_5c,0);
              }
              local_4._0_1_ = 0xc;
              FUN_004acba0((void *)((int)this + 0x114),puVar7,0);
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvStack_58);
              local_4 = CONCAT31(local_4._1_3_,0xb);
              FUN_00619730();
            }
            FUN_005130d0((int)auStack_2c);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052d2f0(auStack_2c);
          break;
        case 0x6c:
          FUN_0052d690(auStack_2c,piVar2,3,*(undefined4 *)((int)this + 0x148));
          local_4 = CONCAT31(local_4._1_3_,2);
          pvVar8 = (void *)0x0;
          FUN_00513120((int)auStack_2c);
          while (iStack_10 != 0) {
            pvVar4 = (void *)FUN_0052bed0((int)auStack_2c);
            pvStack_50 = (void *)((int)pvVar8 + 1);
            if ((*(byte *)((int)pvVar4 + 0x50) & 8) == 0) {
              FUN_004025b0(pvVar4,(uint *)&pvStack_58);
              local_4._0_1_ = 3;
              pvStack_5c = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 4;
              if (pvStack_5c == (void *)0x0) {
                pvStack_5c = (void *)0x0;
              }
              else {
                pvStack_5c = FUN_004421d0(pvStack_5c,(uint *)&pvStack_58);
              }
              local_4._0_1_ = 3;
              puVar3 = (uint *)FUN_00458fe0(pvVar4);
              pvVar8 = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 5;
              if (pvVar8 == (void *)0x0) {
                puVar5 = (uint *)0x0;
              }
              else {
                puVar5 = (uint *)FUN_005fbda0(pvVar8,0x45,0x28,puVar3);
              }
              local_4._0_1_ = 3;
              pvVar8 = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 6;
              if (pvVar8 == (void *)0x0) {
                puStack_64 = (uint *)0x0;
              }
              else {
                puStack_64 = (uint *)FUN_005fbda0(pvVar8,0x45,0x28,puVar5);
              }
              uStack_6c = (undefined1)puVar3[2];
              local_4 = CONCAT31(local_4._1_3_,3);
              bVar1 = true;
              FUN_005fcfe0(puVar5,0,0,0x45,0x28,uStack_6c);
              FUN_005fcfe0(puStack_64,0,0,0x45,0x28,uStack_6c);
              iVar6 = FUN_005fc0e0((int *)puVar3);
              iVar9 = (0x45 - iVar6) / 2;
              iVar6 = FUN_005fc0f0((int *)puVar3);
              iVar6 = (0x28 - iVar6) / 2;
              FUN_005fcc30(puVar3,puVar5,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fcc30(puVar3,puStack_64,iVar9,iVar6,0,0,(undefined4 *)0x0,0);
              FUN_005fd0f0(*(void **)((int)this + 0x15c),puStack_64,iVar9,iVar6);
              FUN_0060be60(pvStack_5c,(int)puVar5,(int)puStack_64);
              if (puVar3 != (uint *)0x0) {
                FUN_005fbfa0(puVar3);
                FUN_00618b60((undefined *)puVar3);
              }
              pvVar8 = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 7;
              if (pvVar8 == (void *)0x0) {
                puVar7 = (undefined4 *)0x0;
              }
              else {
                puVar7 = (undefined4 *)FUN_004acd80(pvVar8,&pvStack_58,0);
              }
              local_4._0_1_ = 3;
              FUN_004acba0((void *)((int)this + 0x114),puVar7,0);
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)pvStack_5c);
              local_4 = CONCAT31(local_4._1_3_,2);
              FUN_00619730();
              piVar2 = local_54;
            }
            pvVar8 = pvStack_50;
            FUN_005130d0((int)auStack_2c);
          }
          if ((int)pvVar8 < piVar2[0x19]) {
            do {
              pvStack_50 = (void *)FUN_00618b70(0x28);
              local_4._0_1_ = 8;
              if (pvStack_50 == (void *)0x0) {
                pvVar4 = (void *)0x0;
              }
              else {
                pvVar4 = FUN_005fbd20(pvStack_50,local_4c,0x232d,10);
              }
              local_4._0_1_ = 2;
              FUN_004ece30(&pvStack_58);
              local_4._0_1_ = 9;
              pvStack_50 = (void *)FUN_00618b70(0x74);
              local_4._0_1_ = 10;
              if (pvStack_50 == (void *)0x0) {
                this_01 = (void *)0x0;
              }
              else {
                this_01 = FUN_004421d0(pvStack_50,(uint *)&pvStack_58);
              }
              local_4._0_1_ = 9;
              FUN_0060be60(this_01,(int)pvVar4,0);
              *(uint *)((int)this_01 + 0x3c) = *(uint *)((int)this_01 + 0x3c) | 4;
              FUN_005f59f0(*(void **)((int)this + 0x178),(int)this_01);
              local_4 = CONCAT31(local_4._1_3_,2);
              FUN_00619730();
              pvVar8 = (void *)((int)pvVar8 + 1);
            } while ((int)pvVar8 < local_54[0x19]);
          }
          local_4 = local_4 & 0xffffff00;
          FUN_0052d710(auStack_2c);
        }
        if (bVar1) {
          *(undefined1 *)(*(int *)((int)this + 0x174) + 0xd4) = uStack_6c;
        }
        FUN_0060a280(*(void **)((int)this + 0x174));
        (**(code **)(**(int **)((int)this + 0x174) + 4))(5);
        local_4 = 0xffffffff;
        FUN_005f2ff0(local_48);
      }
      SetRect(&tStack_3c,0,0x35,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c) + 0x14);
      InvalidateRect(*(HWND *)((int)this + 0x18),&tStack_3c,1);
      *(int *)((int)this + 0x160) = param_1;
      SetFocus(*(HWND *)((int)this + 0x18));
      piVar2 = (int *)0x1;
    }
  }
  ExceptionList = local_c;
  return piVar2;
}

