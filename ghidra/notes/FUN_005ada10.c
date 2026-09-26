
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005ada10(void *this,float param_1)

{
  float fVar1;
  float fVar2;
  bool bVar3;
  undefined3 extraout_var;
  int iVar4;
  int iVar5;
  int *piVar6;
  void *pvVar7;
  undefined3 extraout_var_00;
  int *piVar8;
  uint uVar9;
  uint uVar10;
  int iVar11;
  
  fVar2 = _DAT_0066c284;
  if (*(int *)((int)this + 0xe94) == 1000) {
    fVar1 = *(float *)((int)this + 0xe6c) - param_1;
    *(float *)((int)this + 0xe6c) = fVar1;
    if (fVar2 < fVar1) {
      return;
    }
    *(undefined4 *)((int)this + 0xe94) = 4;
  }
  uVar10 = *(uint *)((int)this + 0xe94);
  if (uVar10 == 4) {
    bVar3 = FUN_005ad930((int)this);
    iVar11 = CONCAT31(extraout_var,bVar3);
    if ((*(int *)((int)this + 0xe68) == 1) && (iVar11 == 0)) {
      if (*(int *)((int)this + 0x9ec) != 0) goto LAB_005adb5a;
      iVar4 = (**(code **)(*(int *)(*(int *)((int)this + 8) + 100) + 0x10))();
      iVar5 = (**(code **)(*(int *)(*(int *)((int)this + 0xc) + 100) + 0x10))();
      if (((iVar4 == 0) || (iVar5 == 0)) || (iVar4 = FUN_005d0ae0(), iVar4 != 0)) goto LAB_005adb5a;
      bVar3 = false;
      for (piVar6 = (int *)(**(code **)(*(int *)(*(int *)((int)this + 8) + 0x70) + 0xc))();
          piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 4))()) {
        iVar4 = (**(code **)(*piVar6 + 0xc))();
        if ((*(int *)(iVar4 + 0x68) == 6) ||
           (iVar4 = (**(code **)(*piVar6 + 0xc))(), *(int *)(iVar4 + 100) == 6)) {
          bVar3 = true;
          break;
        }
      }
      if (bVar3) goto LAB_005adb5a;
      uVar10 = 0x94;
    }
    else {
      if ((*(int *)((int)this + 0xe68) != 0) || (iVar11 != 1)) goto LAB_005adb5a;
      if (*(int *)((int)this + 0x9ec) == 0) {
        uVar10 = 0x92;
      }
      else {
        iVar4 = (**(code **)(*(int *)(*(int *)((int)this + 8) + 100) + 0x10))();
        if (iVar4 == 0) goto LAB_005adb5a;
        uVar10 = 0x11c;
      }
    }
    iVar4 = 1;
    pvVar7 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar7,uVar10,iVar4);
LAB_005adb5a:
    *(int *)((int)this + 0xe68) = iVar11;
    return;
  }
  if (uVar10 == 0) {
    if (*(int *)((int)this + 0x8b4) == 2) {
      return;
    }
    if (*(int *)((int)this + 0x9ec) == 0) {
      *(undefined4 *)((int)this + 0xe74) = 0x20;
      *(undefined4 *)((int)this + 0xe78) = 0x90;
      *(undefined4 *)((int)this + 0xe7c) = 0x93;
      *(undefined4 *)((int)this + 0xe80) = 0x94;
    }
    else {
      *(undefined4 *)((int)this + 0xe74) = 0x9a;
    }
  }
  fVar1 = _DAT_0066c284;
  fVar2 = *(float *)((int)this + 0xe6c) - param_1;
  *(float *)((int)this + 0xe6c) = fVar2;
  if (fVar1 < fVar2) {
    return;
  }
  *(undefined4 *)((int)this + 0xe6c) = DAT_0066c238;
  if (uVar10 < 4) {
    *(undefined4 *)((int)this + uVar10 * 4 + 0xe84) = 1;
    iVar11 = 1;
    uVar10 = *(uint *)((int)this + *(int *)((int)this + 0xe94) * 4 + 0xe74);
    pvVar7 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar7,uVar10,iVar11);
  }
  switch(*(undefined4 *)((int)this + 0xe94)) {
  case 0:
    if (((*(int *)((int)this + 0x9ec) == 0) && (iVar11 = FUN_005a9330(), iVar11 != 0)) &&
       (iVar11 = FUN_005ba420(iVar11), iVar11 != 0)) {
      *(undefined4 *)((int)this + 0xe94) = 1;
    }
    bVar3 = *(int *)((int)this + 0xe94) == 0;
    break;
  case 1:
    iVar11 = FUN_005a9330();
    if (iVar11 != 0) {
      iVar4 = FUN_005ba420(iVar11);
      if ((iVar4 == 0) || (*(int *)(iVar11 + 0x70) == 0)) {
        iVar11 = (**(code **)(*(int *)(*(int *)((int)this + 8) + 100) + 0x10))();
        if ((iVar11 != 0) &&
           ((bVar3 = FUN_005ad930((int)this), CONCAT31(extraout_var_00,bVar3) == 0 &&
            (iVar11 = FUN_005d0ae0(), iVar11 == 0)))) {
          uVar10 = 0;
          uVar9 = 0;
          for (piVar6 = (int *)(**(code **)(*(int *)((int)this + 0x988) + 0x10))();
              piVar6 != (int *)0x0; piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
            piVar8 = (int *)(**(code **)(*piVar6 + 0x10))();
            iVar11 = (**(code **)(*piVar8 + 0x20))();
            if (iVar11 == 1) {
              iVar11 = (**(code **)(*piVar6 + 0x10))();
              if (*(int *)(iVar11 + 0x168) == 0) {
                iVar11 = (**(code **)(*piVar6 + 0x10))();
                if (*(int *)(iVar11 + 0x170) != 10) {
                  uVar10 = uVar10 + 1;
                }
              }
              else {
                uVar9 = uVar9 + 1;
              }
            }
          }
          if (uVar10 <= uVar9) {
            *(undefined4 *)((int)this + 0xe94) = 3;
          }
        }
      }
      else {
        *(undefined4 *)((int)this + 0xe94) = 2;
      }
    }
    bVar3 = *(int *)((int)this + 0xe94) == 1;
    break;
  case 2:
    *(undefined4 *)((int)this + 0xe94) = 1000;
    *(undefined4 *)((int)this + 0xe6c) = DAT_0066c238;
    return;
  case 3:
    *(undefined4 *)((int)this + 0xe94) = 1000;
    *(undefined4 *)((int)this + 0xe6c) = DAT_0066c238;
  default:
    goto switchD_005adc22_default;
  }
  if (bVar3) {
    *(undefined4 *)((int)this + 0xe94) = 1000;
    *(undefined4 *)((int)this + 0xe6c) = DAT_0066c238;
    return;
  }
switchD_005adc22_default:
  return;
}

