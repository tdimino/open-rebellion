
void __fastcall FUN_005ae460(void *param_1)

{
  uint uVar1;
  undefined **ppuVar2;
  bool bVar3;
  int *piVar4;
  int *piVar5;
  int iVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar7;
  int *piVar8;
  int *piVar9;
  void *pvVar10;
  uint uVar11;
  undefined **ppuVar12;
  uint *puVar13;
  int aiStackY_d4 [8];
  undefined4 uStackY_b4;
  undefined4 uStackY_b0;
  undefined4 uStackY_ac;
  int iStackY_a8;
  int iStackY_a4;
  int *piStackY_a0;
  int iStackY_9c;
  undefined4 uStackY_98;
  int *piStackY_94;
  int iStackY_90;
  int iStackY_8c;
  int iStack_6c;
  int iStack_48;
  int iStack_40;
  void *local_3c;
  undefined **appuStack_38 [3];
  uint auStack_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00652f40;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_3c = param_1;
  piVar4 = (int *)(**(code **)(*(int *)((int)param_1 + 0x4c) + 0x10))();
  do {
    if (piVar4 == (int *)0x0) {
      auStack_2c[0] = 0;
      auStack_2c[1] = 0;
      auStack_2c[2] = 0;
      auStack_2c[3] = 0;
      for (piVar4 = (int *)(**(code **)(*(int *)((int)param_1 + 0x4c) + 0x10))();
          piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
        iVar6 = (**(code **)(*piVar4 + 0x10))();
        if (*(int *)(iVar6 + 0x168) == 0) {
          piVar5 = (int *)(**(code **)(*piVar4 + 0x10))();
          iVar6 = (**(code **)(*piVar5 + 0x24))();
          if (iVar6 - 0x1dU < 4) {
            piVar5 = (int *)(**(code **)(*piVar4 + 0x10))();
            iVar6 = (**(code **)(*piVar5 + 0x24))();
            (&piStackY_a0)[iVar6] = (int *)((int)(&piStackY_a0)[iVar6] + 1);
          }
        }
      }
      uVar11 = 0;
      iVar6 = 0;
      do {
        uVar1 = auStack_2c[iVar6];
        auStack_2c[iVar6 + 4] = 0;
        if (uVar11 < uVar1) {
          uVar11 = uVar1;
        }
        if (uVar1 != 0) {
          piVar4 = FUN_005ae330(param_1,(int *)((int)param_1 + 0x4c),iVar6 + 0x1d,iVar6);
          auStack_2c[iVar6 + 4] = (uint)piVar4;
        }
        iVar6 = iVar6 + 1;
      } while (iVar6 < 4);
      iStack_48 = 0;
      do {
        if (auStack_2c[iStack_48] == 0) {
          uVar11 = 0;
          iVar6 = 0;
          iVar7 = 0;
          puVar13 = auStack_2c;
          do {
            if (uVar11 < *puVar13) {
              uVar11 = *puVar13;
              iVar6 = iVar7;
            }
            iVar7 = iVar7 + 1;
            puVar13 = puVar13 + 1;
          } while (iVar7 < 4);
          if (7 < auStack_2c[iVar6]) {
            uVar11 = auStack_2c[iVar6] >> 1;
            auStack_2c[iStack_48] = uVar11;
            auStack_2c[iVar6] = auStack_2c[iVar6] - uVar11;
            FUN_005f52c0(appuStack_38);
            appuStack_38[0] = &PTR_FUN_0066bdd0;
            piVar4 = (int *)auStack_2c[iVar6 + 4];
            piVar5 = (int *)0x0;
            uStack_4 = 0;
            piVar8 = (int *)(**(code **)(piVar4[7] + 0x10))();
            iStack_40 = 0x1d;
            for (uVar11 = auStack_2c[iStack_48]; uVar11 != 0; uVar11 = uVar11 - 1) {
              if (piVar8 != (int *)0x0) {
                piVar5 = (int *)(**(code **)(*piVar8 + 0x10))();
                iStack_40 = (**(code **)(*piVar5 + 0x24))();
                piVar5 = (int *)(**(code **)(*piVar8 + 8))();
                iVar6 = (**(code **)(*piVar8 + 0x10))();
                FUN_005c5f20(appuStack_38,iVar6);
                iVar6 = *piVar4;
                (**(code **)(*piVar8 + 4))();
                (**(code **)(iVar6 + 0x54))();
                iVar6 = *piVar4;
                (**(code **)(*piVar8 + 4))();
                (**(code **)(iVar6 + 0x30))();
                param_1 = local_3c;
              }
              piVar8 = piVar5;
            }
            piVar4 = FUN_005ae330(param_1,(int *)appuStack_38,iStack_40,iStack_48);
            uStack_4 = 0xffffffff;
            auStack_2c[iStack_48 + 4] = (uint)piVar4;
            FUN_005c5ff0(appuStack_38);
          }
        }
        iStack_48 = iStack_48 + 1;
      } while (iStack_48 < 4);
      FUN_005c81d0(*(int **)((int)param_1 + 0x24c));
      pvVar10 = (void *)0x0;
      piVar4 = *(int **)((int)param_1 + 0x24c) + 0x26;
      (**(code **)(**(int **)((int)param_1 + 0x24c) + 0x1c))();
      (**(code **)(**(int **)((int)param_1 + 0x24c) + 0x1c))();
      (**(code **)(**(int **)((int)param_1 + 0x24c) + 0x1c))();
      for (piVar5 = (int *)(**(code **)(*(int *)((int)param_1 + 0x4c) + 0x10))();
          piVar5 != (int *)0x0; piVar5 = (int *)(**(code **)(*piVar5 + 8))()) {
        iVar6 = (**(code **)(*piVar5 + 0x10))();
        if (*(int *)(iVar6 + 0x168) == 1) {
          piVar8 = (int *)(**(code **)(*piVar5 + 0x10))();
          iVar6 = (**(code **)(*piVar8 + 0x24))();
          if (iVar6 - 0x21U < 4) {
            piVar8 = (int *)(**(code **)(*piVar5 + 0x10))();
            iVar6 = (**(code **)(*piVar8 + 0x24))();
            aiStackY_d4[iVar6] = aiStackY_d4[iVar6] + 1;
          }
        }
      }
      ppuVar12 = (undefined **)0x0;
      iVar6 = 0;
      do {
        ppuVar2 = appuStack_38[iVar6 + -6];
        (&iStack_40)[iVar6] = 0;
        if (ppuVar12 < ppuVar2) {
          ppuVar12 = ppuVar2;
        }
        if (ppuVar2 != (undefined **)0x0) {
          iStackY_8c = 0x5ae7b8;
          piVar5 = FUN_005ae330(param_1,(int *)((int)param_1 + 0x4c),iVar6 + 0x21,iVar6);
          (&iStack_40)[iVar6] = (int)piVar5;
        }
        iVar6 = iVar6 + 1;
      } while (iVar6 < 4);
      iStack_6c = 0;
      do {
        if (appuStack_38[iStack_6c + -6] == (undefined **)0x0) {
          uVar11 = 0;
          iVar6 = 0;
          iVar7 = 0;
          puVar13 = (uint *)&stack0xffffffb0;
          do {
            if (uVar11 < *puVar13) {
              uVar11 = *puVar13;
              iVar6 = iVar7;
            }
            iVar7 = iVar7 + 1;
            puVar13 = puVar13 + 1;
          } while (iVar7 < 4);
          if ((undefined **)0x7 < appuStack_38[iVar6 + -6]) {
            ppuVar12 = (undefined **)((uint)appuStack_38[iVar6 + -6] >> 1);
            appuStack_38[iStack_6c + -6] = ppuVar12;
            appuStack_38[iVar6 + -6] = (undefined **)((int)appuStack_38[iVar6 + -6] - (int)ppuVar12)
            ;
            FUN_005f52c0((undefined4 *)&stack0xffffffa4);
            piVar5 = (int *)(&iStack_40)[iVar6];
            auStack_2c[1] = 1;
            piVar8 = (int *)(**(code **)(piVar5[7] + 0x10))();
            iVar6 = 0x21;
            if (piVar8 != (int *)0x0) {
              piVar9 = (int *)(**(code **)(*piVar8 + 0x10))();
              iVar6 = (**(code **)(*piVar9 + 0x24))();
              (**(code **)(*piVar8 + 8))();
              iVar7 = (**(code **)(*piVar8 + 0x10))();
              FUN_005c5f20(&stack0xffffffa4,iVar7);
              iVar7 = *piVar5;
              (**(code **)(*piVar8 + 4))();
              (**(code **)(iVar7 + 0x54))();
              iVar7 = *piVar5;
              (**(code **)(*piVar8 + 4))();
              param_1 = pvVar10;
              (**(code **)(iVar7 + 0x30))();
              pvVar10 = param_1;
            }
            iStackY_8c = 0x5ae8a2;
            piVar5 = FUN_005ae330(param_1,(int *)&stack0xffffffa4,iVar6,iStack_6c);
            (&iStack_40)[iStack_6c] = (int)piVar5;
            auStack_2c[1] = 0xffffffff;
            FUN_005c5ff0((undefined4 *)&stack0xffffffa4);
          }
        }
        iStack_6c = iStack_6c + 1;
      } while (iStack_6c < 4);
      FUN_005c81d0(*(int **)((int)param_1 + 0x250));
      iStackY_8c = 0x5ae8ea;
      iVar6 = (**(code **)(**(int **)((int)param_1 + 0x250) + 0x1c))();
      iStackY_8c = iVar6 + 3;
      piStackY_94 = *(int **)((int)param_1 + 0x250) + 0x28;
      uStackY_98 = 0x5ae903;
      iStackY_90 = iVar6;
      iStackY_9c = (**(code **)(**(int **)((int)param_1 + 0x250) + 0x1c))();
      iStackY_9c = iVar6 + iStackY_9c;
      uStackY_98 = 7;
      piStackY_a0 = *(int **)((int)param_1 + 0x250) + 0x2a;
      iStackY_a4 = 0x5ae91a;
      (**(code **)(**(int **)((int)param_1 + 0x250) + 0x1c))();
      iStackY_a8 = 0x5ae929;
      iStackY_a4 = (int)param_1 + 0x4c;
      (**(code **)(**(int **)((int)param_1 + 0x24c) + 0x24))();
      uStackY_ac = 0x5ae935;
      iStackY_a8 = (int)param_1 + 0x4c;
      (**(code **)(**(int **)((int)param_1 + 0x250) + 0x24))();
      uStackY_ac = 0;
      uStackY_b0 = 0x5ae942;
      (**(code **)(**(int **)((int)param_1 + 0x24c) + 0x14))();
      uStackY_b0 = 0;
      uStackY_b4 = 0x5ae94f;
      (**(code **)(**(int **)((int)param_1 + 0x250) + 0x14))();
      *(undefined4 *)((int)param_1 + 0x264) = *(undefined4 *)(*(int *)((int)param_1 + 0x24c) + 0x3c)
      ;
      *(undefined4 *)((int)param_1 + 0x268) = *(undefined4 *)(*(int *)((int)param_1 + 0x250) + 0x3c)
      ;
      ExceptionList = piVar4;
      return;
    }
    piVar5 = (int *)(**(code **)(*piVar4 + 0x10))();
    iVar6 = (**(code **)(*piVar5 + 0x20))();
    if (iVar6 == 1) {
      bVar3 = FUN_005b69f0((int)piVar5);
      if (CONCAT31(extraout_var,bVar3) == 0) {
        pvVar10 = (void *)(*(int *)((int)param_1 + piVar5[0x5a] * 4 + 0x24c) + 100);
        goto LAB_005ae4d5;
      }
    }
    else {
      pvVar10 = (void *)(*(int *)((int)param_1 + piVar5[0x5a] * 4 + 0x24c) + 0x58);
LAB_005ae4d5:
      FUN_005c5f20(pvVar10,(int)piVar5);
    }
    bVar3 = FUN_005b69f0((int)piVar5);
    if (CONCAT31(extraout_var_00,bVar3) == 0) {
      FUN_005c5f20((void *)((int)param_1 + 100),(int)piVar5);
    }
    piVar4 = (int *)(**(code **)(*piVar4 + 8))();
  } while( true );
}

