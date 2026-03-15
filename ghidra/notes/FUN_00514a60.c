
undefined4 __cdecl FUN_00514a60(uint *param_1,uint *param_2,uint *param_3,void *param_4)

{
  uint *puVar1;
  bool bVar2;
  void *pvVar3;
  int iVar4;
  undefined3 extraout_var;
  void *pvVar5;
  uint uVar6;
  int *piVar7;
  uint *puVar8;
  uint *puVar9;
  uint uVar10;
  int *piVar11;
  undefined4 uVar12;
  bool bVar13;
  int *local_20;
  int *piStack_1c;
  int *local_18;
  uint uStack_14;
  uint uStack_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  puVar8 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00642630;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_20 = (int *)FUN_00504dc0(param_1);
  if (local_20 == (int *)0x0) {
    ExceptionList = local_c;
    return 1;
  }
  pvVar3 = (void *)thunk_FUN_00506e40();
  pvVar5 = param_4;
  puVar9 = param_3;
  bVar13 = pvVar3 != (void *)0x0;
  if (pvVar3 != (void *)0x0) {
    iVar4 = FUN_005457f0(pvVar3,(int *)puVar8,param_2,param_3,param_4);
    if ((iVar4 == 0) || (!bVar13)) {
      bVar13 = false;
    }
    else {
      bVar13 = true;
    }
  }
  pvVar3 = (void *)thunk_FUN_00506e80();
  if ((pvVar3 == (void *)0x0) || (!bVar13)) {
    bVar13 = false;
  }
  else {
    bVar13 = true;
  }
  if (pvVar3 != (void *)0x0) {
    bVar2 = FUN_00560d50(pvVar3,puVar8,param_2,puVar9,pvVar5);
    if ((CONCAT31(extraout_var,bVar2) == 0) || (!bVar13)) {
      bVar13 = false;
    }
    else {
      bVar13 = true;
    }
  }
  if ((*(byte *)(local_20 + 0x14) & 4) == 0) {
    puVar1 = (uint *)(local_20 + 0xe);
    iVar4 = FUN_004ece60(puVar1);
    if (iVar4 != 0) {
      pvVar5 = (void *)FUN_00505750(puVar1);
      if ((pvVar5 == (void *)0x0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
      if (pvVar5 != (void *)0x0) {
        uVar6 = FUN_00529d20(pvVar5,(int *)puVar8,param_2,puVar9,param_4);
        if ((uVar6 == 0) || (!bVar13)) {
          bVar13 = false;
        }
        else {
          bVar13 = true;
        }
      }
    }
  }
  piVar7 = (int *)FUN_00504dc0(param_2);
  if ((piVar7 == (int *)0x0) || (!bVar13)) {
    bVar13 = false;
  }
  else {
    bVar13 = true;
  }
  local_18 = piVar7;
  param_3 = (uint *)FUN_00504dc0(puVar9);
  if ((param_3 == (uint *)0x0) || (!bVar13)) {
    bVar13 = false;
  }
  else {
    bVar13 = true;
  }
  iVar4 = thunk_FUN_00506e60();
  piVar11 = local_20;
  if ((iVar4 == 0) || (!bVar13)) {
    bVar13 = false;
  }
  else {
    bVar13 = true;
  }
  if (iVar4 != 0) {
    uVar6 = FUN_00556430(local_20,param_2,puVar9,param_4);
    if ((uVar6 == 0) || (!bVar13)) {
      bVar13 = false;
    }
    else {
      bVar13 = true;
    }
  }
  puVar8 = param_3;
  if ((*(byte *)(piVar11 + 0x14) & 0x40) != 0) {
    if (piVar7 != (int *)0x0) {
      iVar4 = (**(code **)(*piVar7 + 0xc4))(piVar11,0,param_4);
      if ((iVar4 == 0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
    }
    puVar8 = param_3;
    if (param_3 != (uint *)0x0) {
      iVar4 = (**(code **)(*param_3 + 0xc4))(piVar11,1,param_4);
      if ((iVar4 == 0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
    }
  }
  if ((*(byte *)(piVar11 + 0x14) & 1) != 0) {
    if (piVar7 != (int *)0x0) {
      iVar4 = (**(code **)(*piVar7 + 200))(piVar11,0,param_4);
      if ((iVar4 == 0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
    }
    if (puVar8 != (uint *)0x0) {
      iVar4 = (**(code **)(*puVar8 + 200))(piVar11,1,param_4);
      if ((iVar4 == 0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
    }
  }
  if ((piVar11[0x14] & 0x200U) != 0) {
    if (piVar7 != (int *)0x0) {
      iVar4 = (**(code **)(*piVar7 + 0xcc))(piVar11,0,param_4);
      if ((iVar4 == 0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
    }
    if (puVar8 != (uint *)0x0) {
      iVar4 = (**(code **)(*puVar8 + 0xcc))(piVar11,1,param_4);
      if ((iVar4 == 0) || (!bVar13)) {
        bVar13 = false;
      }
      else {
        bVar13 = true;
      }
    }
  }
  param_1 = (uint *)0x0;
  piStack_1c = (int *)0x0;
  uVar6 = FUN_00518530(piVar7,(int *)&param_1);
  if ((uVar6 == 0) || (!bVar13)) {
    bVar13 = false;
  }
  else {
    bVar13 = true;
  }
  uVar6 = FUN_00518530((int *)puVar8,(int *)&piStack_1c);
  if ((uVar6 == 0) || (!bVar13)) {
    bVar13 = false;
  }
  else {
    bVar13 = true;
  }
  if ((param_1 == (uint *)0x0) || (piStack_1c == (int *)0x0)) {
LAB_00514dab:
    piVar7 = local_20;
    if ((*(byte *)(local_20 + 0x14) & 0x40) != 0) {
      if (param_1 != (uint *)0x0) {
        iVar4 = (**(code **)(*param_1 + 0xd0))(local_20,0,param_4);
        if ((iVar4 == 0) || (!bVar13)) {
          bVar13 = false;
        }
        else {
          bVar13 = true;
        }
      }
      if (piStack_1c != (int *)0x0) {
        iVar4 = (**(code **)(*piStack_1c + 0xd0))(piVar7,1,param_4);
        if ((iVar4 == 0) || (!bVar13)) {
          bVar13 = false;
        }
        else {
          bVar13 = true;
        }
      }
    }
    if ((*(byte *)(piVar7 + 0x14) & 1) != 0) {
      if (param_1 != (uint *)0x0) {
        iVar4 = (**(code **)(*param_1 + 0xd4))(piVar7,0,param_4);
        if ((iVar4 == 0) || (!bVar13)) {
          bVar13 = false;
        }
        else {
          bVar13 = true;
        }
      }
      if (piStack_1c != (int *)0x0) {
        iVar4 = (**(code **)(*piStack_1c + 0xd4))(piVar7,1,param_4);
        if ((iVar4 == 0) || (!bVar13)) {
          bVar13 = false;
        }
        else {
          bVar13 = true;
        }
      }
    }
  }
  else {
    puVar8 = FUN_004025b0(piStack_1c,&uStack_14);
    uStack_4 = 0;
    puVar9 = FUN_004025b0(param_1,(uint *)&param_2);
    uVar6 = *puVar9;
    uVar10 = *puVar8;
    FUN_00619730();
    uStack_4 = 0xffffffff;
    FUN_00619730();
    piVar7 = local_20;
    if (uVar6 != uVar10) goto LAB_00514dab;
  }
  piVar11 = local_18;
  if (bVar13) {
    uStack_14 = 0x10;
    uStack_10 = 0x40;
    bVar2 = true;
    uStack_4 = 1;
    uVar6 = (**(code **)(*piVar7 + 4))();
    if ((uVar6 < uStack_14) || (uStack_10 <= uVar6)) {
      bVar2 = false;
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    piVar11 = local_18;
    if (bVar2) {
      uStack_10 = 0x20;
      uStack_14 = 0x14;
      uStack_4 = 2;
      uVar6 = (**(code **)(*piVar7 + 4))();
      if ((uVar6 < uStack_14) || (uStack_10 <= uVar6)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      uStack_4 = 0xffffffff;
      FUN_00619730();
      piVar11 = local_18;
      if (bVar2) {
        uStack_14 = 0x14;
        uStack_10 = 0x1c;
        uStack_4 = 3;
        uVar6 = (**(code **)(*piVar7 + 4))();
        if ((uVar6 < uStack_14) || (uStack_10 <= uVar6)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        uStack_4 = 0xffffffff;
        FUN_00619730();
        piVar11 = local_18;
        if (bVar2) {
          uStack_14 = 0x18;
          uStack_10 = 0x1c;
          uStack_4 = 4;
          uVar6 = (**(code **)(*piVar7 + 4))();
          if ((uVar6 < uStack_14) || (uStack_10 <= uVar6)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          uStack_4 = 0xffffffff;
          FUN_00619730();
          piVar11 = local_18;
          if (bVar2) {
            param_2 = (uint *)0x0;
            local_20 = (int *)0x0;
            uVar6 = FUN_00518690(local_18,(int *)&param_2);
            uVar10 = FUN_00518690((int *)param_3,(int *)&local_20);
            if ((uVar10 == 0) || (uVar6 == 0)) {
              bVar13 = false;
            }
            else {
              bVar13 = true;
            }
            if ((param_2 != (uint *)0x0) && (local_20 != (int *)0x0)) {
              puVar8 = FUN_004025b0(local_20,&uStack_14);
              uStack_4 = 5;
              puVar9 = FUN_004025b0(param_2,(uint *)&param_3);
              uVar6 = *puVar9;
              uVar10 = *puVar8;
              FUN_00619730();
              uStack_4 = 0xffffffff;
              FUN_00619730();
              if (uVar6 == uVar10) goto LAB_0051505f;
            }
            iVar4 = FUN_005178b0(param_4);
            if ((iVar4 == 0) || (!bVar13)) {
              bVar13 = false;
            }
            else {
              bVar13 = true;
            }
          }
        }
      }
    }
  }
LAB_0051505f:
  if (piVar11 != (int *)0x0) {
    uVar6 = FUN_00517510(piVar11,param_4);
    if ((uVar6 == 0) || (!bVar13)) {
      bVar13 = false;
    }
    else {
      bVar13 = true;
    }
  }
  iVar4 = FUN_005175b0(piVar7,param_4);
  if ((iVar4 == 0) || (!bVar13)) {
    uVar12 = 0;
  }
  else {
    uVar12 = 1;
  }
  ExceptionList = local_c;
  return uVar12;
}

