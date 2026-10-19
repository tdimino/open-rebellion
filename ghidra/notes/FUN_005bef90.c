
void __fastcall FUN_005bef90(int *param_1)

{
  uint uVar1;
  bool bVar2;
  undefined4 *puVar3;
  int iVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  int *piVar8;
  void *pvVar9;
  int iVar10;
  longlong lVar11;
  uint *puVar12;
  void *pvStack_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006537a6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar3 = (undefined4 *)FUN_0059ed70();
  *puVar3 = 0;
  iVar4 = FUN_0059ed70();
  iVar10 = *(int *)(iVar4 + 0x184) + 1;
  *(int *)(iVar4 + 0x184) = iVar10;
  if (iVar10 < 0) {
    *(int *)(iVar4 + 0x184) = 0;
  }
  piVar5 = (int *)(**(code **)(param_1[0x268] + 0x10))();
  do {
    if (piVar5 == (int *)0x0) {
      lVar11 = __ftol();
      param_1[param_1[0x27b] + 0x21c] = (int)lVar11;
      pvStack_1c = (void *)FUN_00618b70(0x220);
      uStack_4 = 0;
      if (pvStack_1c == (void *)0x0) {
        pvVar9 = (void *)0x0;
      }
      else {
        uVar1 = param_1[0x21f];
        puVar12 = (uint *)0x0;
        lVar11 = __ftol();
        pvVar9 = FUN_005ce500(pvStack_1c,(float)uVar1,0x4a,param_1[0x27b],(int)lVar11,puVar12);
      }
      uStack_4 = 0xffffffff;
      (**(code **)(*param_1 + 0x2c))(pvVar9,1);
      pvVar9 = (void *)FUN_00618b70(0x220);
      pvStack_c = (void *)0x1;
      if (pvVar9 == (void *)0x0) {
        pvVar9 = (void *)0x0;
      }
      else {
        iVar4 = param_1[0x27b];
        uVar1 = param_1[0x21f];
        pvStack_1c = (void *)param_1[0x22b];
        puVar12 = (uint *)0x0;
        lVar11 = __ftol();
        pvVar9 = FUN_005ce500(pvVar9,(float)uVar1,0x4a,(uint)(iVar4 == 0),(int)lVar11,puVar12);
      }
      pvStack_c = (void *)0xffffffff;
      (**(code **)(*param_1 + 0x2c))(pvVar9,1);
      ExceptionList = pvStack_1c;
      return;
    }
    piVar6 = (int *)(**(code **)(*piVar5 + 0x10))();
    iVar4 = (**(code **)(*piVar6 + 0x20))();
    if ((iVar4 == 1) &&
       ((((iVar4 = FUN_005ad870(piVar6), iVar4 == 1 || (iVar4 = piVar6[0x5c], iVar4 == 1)) ||
         (iVar4 == 5)) || (iVar4 == 9)))) {
      bVar2 = false;
      for (piVar7 = (int *)(**(code **)(param_1[0x268] + 0x10))(); piVar7 != (int *)0x0;
          piVar7 = (int *)(**(code **)(*piVar7 + 8))()) {
        if (bVar2) goto LAB_005bf116;
        piVar8 = (int *)(**(code **)(*piVar7 + 0x10))();
        iVar4 = (**(code **)(*piVar8 + 0x20))();
        if ((iVar4 == 0) &&
           (iVar4 = (**(code **)(*piVar7 + 0x10))(), *(int *)(iVar4 + 0x168) == piVar6[0x5a])) {
          iVar4 = (**(code **)(*piVar7 + 0x10))();
          iVar10 = FUN_005ad870(iVar4);
          if ((((iVar10 == 1) || ((iVar10 = *(int *)(iVar4 + 0x170), iVar10 == 7 || (iVar10 == 8))))
              || (iVar10 == 9)) &&
             (iVar4 = FUN_005b84a0((void *)(iVar4 + 0x418),(int)piVar6), iVar4 == 1)) {
            bVar2 = true;
          }
        }
      }
      if ((((!bVar2) && (iVar4 = FUN_005a9330(), iVar4 != 0)) &&
          ((iVar10 = FUN_005ba420(iVar4), iVar10 != 0 || (*(int *)(iVar4 + 0x78) != 0)))) &&
         (piVar6[0x5a] == 1)) {
        FUN_005b84a0((void *)(iVar4 + 0x1c),(int)piVar6);
      }
    }
LAB_005bf116:
    piVar5 = (int *)(**(code **)(*piVar5 + 8))();
  } while( true );
}

