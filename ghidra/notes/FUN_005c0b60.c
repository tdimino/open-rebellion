
void __fastcall FUN_005c0b60(int *param_1)

{
  bool bVar1;
  undefined4 *puVar2;
  int iVar3;
  int *piVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  void *pvVar8;
  int iVar9;
  longlong lVar10;
  uint *puVar11;
  void *local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006538ab;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar2 = (undefined4 *)FUN_0059ed70();
  *puVar2 = 0;
  iVar3 = FUN_0059ed70();
  iVar9 = *(int *)(iVar3 + 0x184) + 1;
  *(int *)(iVar3 + 0x184) = iVar9;
  if (iVar9 < 0) {
    *(int *)(iVar3 + 0x184) = 0;
  }
  piVar4 = (int *)(**(code **)(param_1[0x268] + 0x10))();
  do {
    if (piVar4 == (int *)0x0) {
      local_14 = (void *)param_1[0x21f];
      lVar10 = __ftol();
      param_1[param_1[0x27b] + 0x21c] = (int)lVar10;
      pvVar8 = (void *)FUN_00618b70(0x220);
      uStack_4 = 0;
      if (pvVar8 == (void *)0x0) {
        pvVar8 = (void *)0x0;
      }
      else {
        local_14 = (void *)(float)(uint)param_1[0x21f];
        puVar11 = (uint *)0x0;
        lVar10 = __ftol();
        pvVar8 = FUN_005ce500(pvVar8,local_14,0x4a,param_1[0x27b],(int)lVar10,puVar11);
      }
      uStack_4 = 0xffffffff;
      (**(code **)(*param_1 + 0x2c))(pvVar8,1);
      ExceptionList = local_14;
      return;
    }
    piVar5 = (int *)(**(code **)(*piVar4 + 0x10))();
    iVar3 = (**(code **)(*piVar5 + 0x20))();
    if ((iVar3 == 1) &&
       ((((iVar3 = FUN_005ad870(piVar5), iVar3 == 1 || (iVar3 = piVar5[0x5c], iVar3 == 1)) ||
         (iVar3 == 5)) || (iVar3 == 9)))) {
      bVar1 = false;
      for (piVar6 = (int *)(**(code **)(param_1[0x268] + 0x10))(); piVar6 != (int *)0x0;
          piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
        if (bVar1) goto LAB_005c0ce1;
        piVar7 = (int *)(**(code **)(*piVar6 + 0x10))();
        iVar3 = (**(code **)(*piVar7 + 0x20))();
        bVar1 = false;
        if ((iVar3 == 0) &&
           (iVar3 = (**(code **)(*piVar6 + 0x10))(), *(int *)(iVar3 + 0x168) == piVar5[0x5a])) {
          iVar3 = (**(code **)(*piVar6 + 0x10))();
          iVar9 = FUN_005ad870(iVar3);
          if ((((iVar9 == 1) || ((iVar9 = *(int *)(iVar3 + 0x170), iVar9 == 7 || (iVar9 == 8)))) ||
              (iVar9 == 9)) &&
             (iVar3 = FUN_005b84a0((void *)(iVar3 + 0x418),(int)piVar5), iVar3 == 1)) {
            bVar1 = true;
          }
        }
      }
      if ((((!bVar1) && (iVar3 = FUN_005a9330(), iVar3 != 0)) &&
          ((iVar9 = FUN_005ba420(iVar3), iVar9 != 0 || (*(int *)(iVar3 + 0x78) != 0)))) &&
         (piVar5[0x5a] == 1)) {
        FUN_005b84a0((void *)(iVar3 + 0x1c),(int)piVar5);
      }
    }
LAB_005c0ce1:
    piVar4 = (int *)(**(code **)(*piVar4 + 8))();
  } while( true );
}

