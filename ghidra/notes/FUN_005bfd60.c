
undefined4 __thiscall FUN_005bfd60(void *this,float param_1,int param_2)

{
  int *piVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  int iVar5;
  void *pvVar6;
  undefined4 uVar7;
  void *this_00;
  int iVar8;
  int iVar9;
  undefined1 local_18;
  void *pvStack_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_0065384b;
  pvStack_c = ExceptionList;
  local_18 = 0x47;
  if (param_2 == 1) {
    local_18 = 0x49;
  }
  iVar8 = 0;
  pvVar6 = (void *)((int)this + param_2 * 0xc + 0x908);
  ExceptionList = &pvStack_c;
  for (piVar1 = (int *)thunk_FUN_005f5060((int)pvVar6); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    if ((float)piVar1[8] == param_1) {
      iVar8 = iVar8 + 1;
    }
  }
  this_00 = (void *)((int)this + param_2 * 0xc + 0x938);
  iVar9 = 0;
  for (piVar1 = (int *)thunk_FUN_005f5060((int)this_00); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 8))()) {
    if ((float)piVar1[8] == param_1) {
      iVar9 = iVar9 + 1;
    }
  }
  do {
    piVar1 = FUN_005ce600(pvVar6);
    if (piVar1 == (int *)0x0) {
      iVar2 = 0;
    }
    else {
      iVar2 = FUN_005970d0(piVar1 + 8,piVar1[7],3);
      FUN_00597170();
      for (piVar3 = (int *)thunk_FUN_005f5060((int)this_00); piVar3 != (int *)0x0;
          piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
        iVar4 = (**(code **)(*piVar3 + 0x1c))();
        iVar5 = (**(code **)(*piVar1 + 0x1c))();
        if (iVar4 == iVar5) goto LAB_005bfe78;
      }
      FUN_005f5440(this_00,piVar1);
    }
LAB_005bfe78:
  } while (iVar2 != 0);
  pvVar6 = (void *)FUN_00618b70(0x220);
  if (pvVar6 == (void *)0x0) {
    piVar1 = (int *)0x0;
  }
  else {
    iStack_4 = iVar2;
    piVar1 = FUN_005ce500(pvVar6,param_1,local_18,iVar8 + 1 + iVar9,0,(uint *)0x0);
  }
  iStack_4 = 0xffffffff;
  uVar7 = 0;
  if (piVar1 != (int *)0x0) {
    uVar7 = FUN_005970d0(piVar1 + 8,piVar1[7],3);
    for (piVar3 = (int *)thunk_FUN_005f5060((int)this_00); piVar3 != (int *)0x0;
        piVar3 = (int *)(**(code **)(*piVar3 + 8))()) {
      iVar8 = (**(code **)(*piVar3 + 0x1c))();
      iVar9 = (**(code **)(*piVar1 + 0x1c))();
      if (iVar8 == iVar9) goto LAB_005bff0f;
    }
    FUN_005f5440(this_00,piVar1);
  }
LAB_005bff0f:
  FUN_00597170();
  ExceptionList = pvStack_c;
  return uVar7;
}

