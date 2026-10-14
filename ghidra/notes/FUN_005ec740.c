
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 FUN_005ec740(void *param_1,int *param_2,int param_3,float param_4)

{
  float fVar1;
  bool bVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  undefined4 uVar6;
  undefined4 *puVar7;
  undefined4 uVar8;
  int iVar9;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int iVar10;
  void *pvVar11;
  void *pvVar12;
  float10 fVar13;
  float *pfVar14;
  float fStack_30;
  undefined4 uStack_2c;
  float fStack_28;
  undefined4 *puStack_24;
  undefined **appuStack_20 [2];
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006553b6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar3 = FUN_005f5060(param_3);
  if ((iVar3 == 0) || (iVar3 = (**(code **)(*param_2 + 0xc))(), iVar3 == 0)) {
LAB_005ecabe:
    uVar8 = 0;
  }
  else {
    FUN_005ca440(auStack_18,param_2);
    uStack_4 = 0;
    for (iVar3 = FUN_005f5060(param_3); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      iVar4 = FUN_005c91c0(param_1,*(undefined4 *)(iVar3 + 0x1c));
      if (iVar4 != 0) {
        FUN_005f4ed0(appuStack_20);
        appuStack_20[0] = &PTR_FUN_0066da60;
        uStack_4 = CONCAT31(uStack_4._1_3_,1);
        fStack_30 = 0.0;
        uStack_2c = 0;
        for (piVar5 = (int *)thunk_FUN_005f5060((int)auStack_18); piVar5 != (int *)0x0;
            piVar5 = (int *)(**(code **)(*piVar5 + 4))()) {
          uVar8 = *(undefined4 *)(iVar3 + 0x1c);
          puVar7 = &uStack_2c;
          fStack_30 = 0.0;
          uStack_2c = 0;
          pfVar14 = &fStack_30;
          uVar6 = (**(code **)(*piVar5 + 0xc))();
          fVar13 = FUN_005c9160(param_1,uVar8,uVar6,pfVar14,puVar7);
          if ((fVar13 < (float10)_DAT_0066da54) || (param_4 < (float)fVar13)) {
            puVar7 = (undefined4 *)FUN_00618b70(0x20);
            uStack_4 = CONCAT31(uStack_4._1_3_,3);
            puStack_24 = puVar7;
            if (puVar7 == (undefined4 *)0x0) goto LAB_005ec8c6;
            uVar8 = (**(code **)(*piVar5 + 0xc))();
            fVar1 = _DAT_0066da58 - fStack_30;
            FUN_005f5c10(puVar7);
            puVar7[6] = fVar1;
            puVar7[7] = uVar8;
            *puVar7 = &PTR_FUN_0066da5c;
          }
          else {
            puVar7 = (undefined4 *)FUN_00618b70(0x20);
            uStack_4 = CONCAT31(uStack_4._1_3_,2);
            puStack_24 = puVar7;
            if (puVar7 == (undefined4 *)0x0) {
LAB_005ec8c6:
              puVar7 = (undefined4 *)0x0;
            }
            else {
              uVar8 = (**(code **)(*piVar5 + 0xc))();
              FUN_005f5c10(puVar7);
              puVar7[6] = (float)fVar13;
              puVar7[7] = uVar8;
              *puVar7 = &PTR_FUN_0066da5c;
            }
          }
          uStack_4 = CONCAT31(uStack_4._1_3_,1);
          FUN_005f4f10(appuStack_20,(int)puVar7);
        }
        iVar4 = FUN_005f5080((int)appuStack_20);
        iVar9 = FUN_005f5060((int)appuStack_20);
        if (iVar9 == 0) {
          uStack_4 = uStack_4 & 0xffffff00;
          appuStack_20[0] = &PTR_FUN_0066da60;
          FUN_005f4f00(appuStack_20);
          uStack_4 = 0xffffffff;
          FUN_005ca3e0(auStack_18);
          goto LAB_005ecabe;
        }
        if (*(float *)(iVar9 + 0x18) == *(float *)(iVar4 + 0x18)) {
          bVar2 = FUN_005f2e40((void *)(iVar9 + 0x1c),(int *)(iVar4 + 0x1c));
          iVar10 = CONCAT31(extraout_var,bVar2);
        }
        else {
          iVar10 = 0;
        }
        if (iVar10 != 0) {
          pvVar11 = *(void **)(iVar9 + 0x1c);
          iVar3 = *(int *)(iVar3 + 0x1c);
          pvVar12 = (void *)FUN_005aaf90();
          FUN_005a8fc0(pvVar12,iVar3,pvVar11);
          iVar3 = FUN_005f5060((int)appuStack_20);
          FUN_005f54a0(auStack_18,*(uint *)(*(int *)(iVar3 + 0x1c) + 0x18));
          uStack_4 = uStack_4 & 0xffffff00;
          appuStack_20[0] = &PTR_FUN_0066da60;
          FUN_005f4f00(appuStack_20);
          uStack_4 = 0xffffffff;
          FUN_005ca3e0(auStack_18);
          ExceptionList = pvStack_c;
          return 1;
        }
        fVar13 = FUN_005cb650(*(void **)(iVar9 + 0x1c),*(int **)(iVar3 + 0x1c));
        fStack_28 = (float)fVar13;
        for (; iVar4 != 0; iVar4 = FUN_005f5c60(iVar4)) {
          if (*(float *)(iVar4 + 0x18) == *(float *)(iVar9 + 0x18)) {
            bVar2 = FUN_005f2e40((void *)(iVar4 + 0x1c),(int *)(iVar9 + 0x1c));
            iVar10 = CONCAT31(extraout_var_00,bVar2);
          }
          else {
            iVar10 = 0;
          }
          if (iVar10 != 0) break;
          fVar13 = FUN_005c9160(param_1,*(undefined4 *)(iVar3 + 0x1c),*(undefined4 *)(iVar4 + 0x1c),
                                &fStack_28,&uStack_2c);
          if (((float10)_DAT_0066da54 <= fVar13) && (fVar13 <= (float10)param_4)) {
            iVar9 = FUN_005f5080((int)appuStack_20);
            goto joined_r0x005ec9bb;
          }
        }
LAB_005eca10:
        uStack_4 = uStack_4 & 0xffffff00;
        appuStack_20[0] = &PTR_FUN_0066da60;
        FUN_005f4f00(appuStack_20);
      }
    }
    for (piVar5 = (int *)thunk_FUN_005f5060((int)auStack_18); piVar5 != (int *)0x0;
        piVar5 = (int *)(**(code **)(*piVar5 + 4))()) {
      iVar3 = FUN_005f5060(param_3);
      if (iVar3 == 0) {
LAB_005eca7a:
        iVar3 = FUN_005f5060(param_3);
        iVar3 = *(int *)(iVar3 + 0x1c);
        pvVar11 = (void *)(**(code **)(*piVar5 + 0xc))();
      }
      else {
        do {
          iVar4 = FUN_005c91c0(param_1,*(undefined4 *)(iVar3 + 0x1c));
          if (iVar4 == 1) break;
          iVar3 = *(int *)(iVar3 + 0x10);
        } while (iVar3 != 0);
        if (iVar3 == 0) goto LAB_005eca7a;
        iVar3 = *(int *)(iVar3 + 0x1c);
        pvVar11 = (void *)(**(code **)(*piVar5 + 0xc))();
      }
      pvVar12 = (void *)FUN_005aaf90();
      FUN_005a8fc0(pvVar12,iVar3,pvVar11);
    }
    uStack_4 = 0xffffffff;
    FUN_005ca3e0(auStack_18);
    uVar8 = 1;
  }
  ExceptionList = pvStack_c;
  return uVar8;
joined_r0x005ec9bb:
  if (iVar9 == 0) goto LAB_005eca10;
  pvVar11 = *(void **)(iVar9 + 0x1c);
  iVar10 = *(int *)(iVar3 + 0x1c);
  pvVar12 = (void *)FUN_005aaf90();
  FUN_005a8fc0(pvVar12,iVar10,pvVar11);
  FUN_005f54a0(auStack_18,*(uint *)(*(int *)(iVar9 + 0x1c) + 0x18));
  if (*(float *)(iVar9 + 0x18) == *(float *)(iVar4 + 0x18)) {
    bVar2 = FUN_005f2e40((int *)(iVar9 + 0x1c),(int *)(iVar4 + 0x1c));
    iVar10 = CONCAT31(extraout_var_01,bVar2);
  }
  else {
    iVar10 = 0;
  }
  if (iVar10 != 0) goto LAB_005eca10;
  iVar9 = FUN_005f5c60(iVar9);
  goto joined_r0x005ec9bb;
}

