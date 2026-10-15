
void __thiscall FUN_00467f10(int *param_1,uint param_2,uint param_3)

{
  void **ppvVar1;
  short sVar2;
  uint uVar3;
  int iVar4;
  int *piVar5;
  undefined2 *puVar6;
  int *piVar7;
  bool bVar8;
  undefined2 uStack_38;
  undefined2 uStack_36;
  undefined2 uStack_34;
  undefined2 uStack_32;
  undefined2 uStack_30;
  undefined2 uStack_2e;
  undefined2 uStack_2c;
  undefined2 uStack_2a;
  undefined2 uStack_28;
  undefined2 uStack_26;
  undefined4 uStack_24;
  int iStack_20;
  undefined4 uStack_1c;
  int iStack_18;
  int iStack_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00631ba0;
  pvStack_c = ExceptionList;
  uVar3 = param_2 & 0xffff;
  switch(uVar3) {
  case 0x28:
    ExceptionList = &pvStack_c;
    if ((param_1[0x46] == 2) && (ExceptionList = &pvStack_c, param_1[0x5c] != 0)) {
      ExceptionList = &pvStack_c;
      *(undefined4 *)(*(int *)(param_1[0x5c] + 0x68) + 0x24) = 1;
      *(undefined4 *)(param_1[0x5c] + 0x38) = 10;
    }
    FUN_00468fb0(param_1,uVar3);
    (**(code **)(*param_1 + 0x30))();
    break;
  case 0x65:
  case 0x68:
    ExceptionList = &pvStack_c;
    if ((param_1[0x46] == 2) && (ExceptionList = &pvStack_c, param_1[0x5c] != 0)) {
      ExceptionList = &pvStack_c;
      *(undefined4 *)(*(int *)(param_1[0x5c] + 0x68) + 0x24) = 1;
      *(undefined4 *)(param_1[0x5c] + 0x38) = 10;
    }
    FUN_00468fb0(param_1,uVar3);
    break;
  case 0x66:
    if (param_1[0x56] == 0) {
      ExceptionList = &pvStack_c;
      uVar3 = FUN_0041d270();
      FUN_0041d290(uVar3 ^ 4);
    }
    else {
      ExceptionList = &pvStack_c;
      uVar3 = FUN_0041d250(param_1[0x56]);
      FUN_0041d230(param_1[0x56],uVar3 ^ 4);
    }
    piVar7 = (int *)FUN_00607e90(param_1,0x66);
    if (piVar7 == (int *)0x0) {
      ExceptionList = pvStack_c;
      return;
    }
    (**(code **)(*piVar7 + 0xc))();
    if ((uVar3 >> 2 & 1) == 0) {
      uStack_38 = 0x8006;
      uStack_36 = DAT_0065d424;
      FUN_00600c40(param_1,piVar7,&uStack_38);
      if (*(int *)(param_1[0x45] + 0x9c) != 1) {
        uVar3 = 0x2ac9;
        goto LAB_00468082;
      }
      uVar3 = 0x2ac7;
    }
    else {
      param_2 = CONCAT22(DAT_0065d424,0x8005);
      FUN_00600c40(param_1,piVar7,&param_2);
      if (*(int *)(param_1[0x45] + 0x9c) != 1) {
        uVar3 = 0x2a7a;
LAB_00468082:
        FUN_00603150(piVar7,0,uVar3);
        FUN_00603150(piVar7,1,0x2a7b);
        ExceptionList = pvStack_c;
        return;
      }
      uVar3 = 0x2a78;
    }
    FUN_00603150(piVar7,0,uVar3);
    FUN_00603150(piVar7,1,0x2a79);
    break;
  case 0x67:
  case 0x97:
    piVar7 = (int *)0x0;
    iStack_20 = 1;
    if (param_1[0x46] == 1) {
      ExceptionList = &pvStack_c;
      piVar5 = (int *)(**(code **)(*(int *)param_1[0x5b] + 8))();
      bVar8 = false;
      while (piVar5 != (int *)0x0) {
        if (bVar8) goto LAB_004686a2;
        bVar8 = (*(byte *)(piVar5 + 0xf) & 1) == 0;
        if (bVar8) {
          piVar5 = (int *)(**(code **)(*piVar5 + 0xc))();
        }
        bVar8 = !bVar8;
      }
      if (bVar8) {
LAB_004686a2:
        piVar7 = (int *)piVar5[0x1a];
      }
    }
    else {
      ExceptionList = &pvStack_c;
      if (param_1[0x46] == 2) {
        piVar7 = *(int **)(param_1[0x5c] + 0x68);
        ExceptionList = &pvStack_c;
        *(undefined4 *)(param_1[0x5c] + 0x6c) = 0;
      }
    }
    if (piVar7 == (int *)0x0) {
      ExceptionList = pvStack_c;
      return;
    }
    FUN_004f26d0(&param_3,piVar7 + 0x17);
    piVar7[9] = 1;
    uStack_4 = 3;
    iVar4 = (**(code **)(*piVar7 + 0x10))();
    if (iVar4 == 3) {
LAB_00468888:
      iVar4 = FUN_004ece60(&param_3);
      if (iVar4 == 0) {
LAB_004686e8:
        iVar4 = 0;
      }
      else {
        FUN_00429440((void *)param_1[0x45],&param_3);
        iVar4 = iStack_20;
      }
    }
    else {
      if (iVar4 != 4) {
        if (iVar4 == 5) goto LAB_00468888;
        goto LAB_004686e8;
      }
      FUN_004f26d0(&uStack_24,piVar7 + 0x18);
      uStack_4._0_1_ = 4;
      FUN_0041d6b0(&param_3,&uStack_24);
      uStack_4 = CONCAT31(uStack_4._1_3_,3);
      FUN_00619730();
      iVar4 = iStack_20;
    }
    if (iVar4 != 0) {
      PostMessageA((HWND)param_1[6],0x111,0x28,0);
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
    break;
  case 0x6e:
    sVar2 = (short)(param_3 >> 0x10);
    if ((sVar2 == 0x309) && (param_1[0x46] == 1)) {
      ExceptionList = &pvStack_c;
      FUN_0060a790(&uStack_1c,1);
      uStack_4 = 1;
      FUN_00609410((void *)param_1[0x5a],&uStack_1c);
      if (iStack_18 != 0) {
        (**(code **)(*param_1 + 0x48))(0x65,0,0);
      }
      uStack_4 = 0xffffffff;
      FUN_0060a810(&uStack_1c);
      ExceptionList = pvStack_c;
      return;
    }
    if (sVar2 != 0x29b) {
      return;
    }
    if (param_1[0x46] != 1) {
      return;
    }
    ExceptionList = &pvStack_c;
    FUN_0060a790(&uStack_1c,1);
    uStack_4 = 2;
    FUN_00609410((void *)param_1[0x5a],&uStack_1c);
    piVar7 = (int *)FUN_00607e90(param_1,0x67);
    if ((piVar7 != (int *)0x0) && ((**(code **)(*piVar7 + 0x10))(), iStack_18 == 1)) {
      iVar4 = FUN_0060a860((void *)param_1[0x5b],*(int *)(iStack_14 + 0xc));
      piVar5 = *(int **)(iVar4 + 0x68);
      if (piVar5 != (int *)0x0) {
        iVar4 = (**(code **)(*piVar5 + 0x10))();
        if ((iVar4 == 3) ||
           ((iVar4 = (**(code **)(*piVar5 + 0x10))(), iVar4 == 5 &&
            (iVar4 = FUN_004ece60((uint *)(piVar5 + 0x17)), iVar4 != 0)))) {
          if (*(int *)(param_1[0x45] + 0x9c) == 1) {
            FUN_00603150(piVar7,0,0x2916);
            uVar3 = 0x2917;
          }
          else {
            FUN_00603150(piVar7,0,0x2918);
            uVar3 = 0x2919;
          }
          FUN_00603150(piVar7,1,uVar3);
          uStack_28 = 0x8030;
          uStack_26 = DAT_0065d424;
          puVar6 = &uStack_28;
        }
        else {
          iVar4 = (**(code **)(*piVar5 + 0x10))();
          if (iVar4 != 4) goto LAB_00468633;
          if (*(int *)(param_1[0x45] + 0x9c) == 1) {
            FUN_00603150(piVar7,0,0x2d20);
            uVar3 = 0x2d21;
          }
          else {
            FUN_00603150(piVar7,0,0x2d47);
            uVar3 = 0x2d48;
          }
          FUN_00603150(piVar7,1,uVar3);
          uStack_2c = 0x8031;
          uStack_2a = DAT_0065d424;
          puVar6 = &uStack_2c;
        }
        FUN_00600c40(param_1,piVar7,puVar6);
        (**(code **)(*piVar7 + 0xc))();
      }
    }
LAB_00468633:
    uStack_4 = 0xffffffff;
    FUN_0060a810(&uStack_1c);
    break;
  case 0x78:
    ExceptionList = &pvStack_c;
    FUN_004697b0(param_1,param_3 >> 0x10);
    goto LAB_00468829;
  case 0x90:
    ExceptionList = &pvStack_c;
    for (piVar7 = (int *)(**(code **)(*(int *)param_1[0x5b] + 8))(); piVar7 != (int *)0x0;
        piVar7 = (int *)(**(code **)(*piVar7 + 0xc))()) {
      if (piVar7[0x1b] != 0) {
        piVar7[0xf] = piVar7[0xf] | 1;
      }
    }
    SetFocus(*(HWND *)(param_1[0x5a] + 0x18));
    FUN_0060a280((void *)param_1[0x5a]);
    FUN_0060a790(&uStack_1c,1);
    uStack_4 = 0;
    FUN_00609410((void *)param_1[0x5a],&uStack_1c);
    piVar7 = (int *)FUN_00607e90(param_1,0x67);
    if ((piVar7 != (int *)0x0) && ((**(code **)(*piVar7 + 0x10))(), iStack_18 == 1)) {
      iVar4 = FUN_0060a860((void *)param_1[0x5b],*(int *)(iStack_14 + 0xc));
      piVar5 = *(int **)(iVar4 + 0x68);
      if (piVar5 != (int *)0x0) {
        iVar4 = (**(code **)(*piVar5 + 0x10))();
        if ((iVar4 == 3) ||
           ((iVar4 = (**(code **)(*piVar5 + 0x10))(), iVar4 == 5 &&
            (iVar4 = FUN_004ece60((uint *)(piVar5 + 0x17)), iVar4 != 0)))) {
          if (*(int *)(param_1[0x45] + 0x9c) == 1) {
            FUN_00603150(piVar7,0,0x2916);
            uVar3 = 0x2917;
          }
          else {
            FUN_00603150(piVar7,0,0x2918);
            uVar3 = 0x2919;
          }
          FUN_00603150(piVar7,1,uVar3);
          uStack_30 = 0x8030;
          uStack_2e = DAT_0065d424;
          puVar6 = &uStack_30;
        }
        else {
          iVar4 = (**(code **)(*piVar5 + 0x10))();
          if (iVar4 != 4) goto LAB_004683a3;
          if (*(int *)(param_1[0x45] + 0x9c) == 1) {
            FUN_00603150(piVar7,0,0x2d20);
            uVar3 = 0x2d21;
          }
          else {
            FUN_00603150(piVar7,0,0x2d47);
            uVar3 = 0x2d48;
          }
          FUN_00603150(piVar7,1,uVar3);
          uStack_34 = 0x8031;
          uStack_32 = DAT_0065d424;
          puVar6 = &uStack_34;
        }
        FUN_00600c40(param_1,piVar7,puVar6);
        (**(code **)(*piVar7 + 0xc))();
      }
    }
LAB_004683a3:
    uStack_4 = 0xffffffff;
    FUN_0060a810(&uStack_1c);
    break;
  case 0x91:
    if (param_1[0x46] == 1) {
      iVar4 = 0;
      ExceptionList = &pvStack_c;
      piVar7 = (int *)(**(code **)(*(int *)param_1[0x5b] + 8))();
      while (piVar5 = piVar7, piVar5 != (int *)0x0) {
        piVar7 = (int *)(**(code **)(*piVar5 + 0xc))();
        if ((*(byte *)(piVar5 + 0xf) & 1) != 0) {
          if ((iVar4 == 0) && (iVar4 = (**(code **)(*piVar5 + 0xc))(), iVar4 == 0)) {
            iVar4 = (**(code **)(*piVar5 + 0x10))();
          }
          FUN_005f5ac0((void *)param_1[0x5b],(int)piVar5);
          FUN_0041d1d0(piVar5[3]);
          if (piVar5 != (int *)0x0) {
            (**(code **)*piVar5)(1);
          }
        }
      }
      SetFocus(*(HWND *)(param_1[0x5a] + 0x18));
      if (iVar4 != 0) {
        FUN_00609500((void *)param_1[0x5a],*(int *)(iVar4 + 0xc),1);
      }
      FUN_0060a280((void *)param_1[0x5a]);
    }
    else if ((int *)param_1[0x5c] != (int *)0x0) {
      ExceptionList = &pvStack_c;
      iVar4 = (**(code **)(*(int *)param_1[0x5c] + 0xc))();
      if (iVar4 == 0) {
        iVar4 = (**(code **)(*(int *)param_1[0x5c] + 0x10))();
      }
      FUN_005f5ac0((void *)param_1[0x5b],param_1[0x5c]);
      FUN_0041d1d0(*(uint *)(param_1[0x5c] + 0xc));
      if (iVar4 == 0) {
        FUN_00468fb0(param_1,0x65);
      }
      else {
        param_1[0x5c] = iVar4;
        FUN_00469de0(param_1,iVar4);
      }
    }
    break;
  case 0x96:
    ExceptionList = &pvStack_c;
    *(undefined4 *)(*(int *)(param_1[0x5c] + 0x68) + 0x24) = 1;
    *(undefined4 *)(param_1[0x5c] + 0x38) = 10;
    iVar4 = (**(code **)(*(int *)param_1[0x5c] + 0xc))();
    if (iVar4 == 0) {
      ExceptionList = pvStack_c;
      return;
    }
    iVar4 = (**(code **)(*(int *)param_1[0x5c] + 0xc))();
    param_1[0x5c] = iVar4;
    FUN_00469de0(param_1,iVar4);
    iVar4 = param_1[0x77];
    goto LAB_0046882f;
  case 0x98:
    if (param_1[0x46] != 2) {
      if (param_1[0x46] != 3) {
        return;
      }
      ExceptionList = &pvStack_c;
      sVar2 = FUN_005f3040(param_1[0x77] + 0xa0);
      if (sVar2 == 0) {
        ExceptionList = pvStack_c;
        return;
      }
      FUN_005f2f90(&uStack_1c,param_1[0x77] + 0xa0);
      uStack_4 = 5;
      FUN_0041d2b0((int)&uStack_1c);
      (**(code **)(*param_1 + 0x48))(0x65,0,0);
      SetFocus(*(HWND *)(param_1[0x5a] + 0x18));
      uStack_4 = 0xffffffff;
      FUN_005f2ff0(&uStack_1c);
      ExceptionList = pvStack_c;
      return;
    }
    ExceptionList = &pvStack_c;
    *(undefined4 *)(param_1[0x5c] + 0x6c) = 0;
    *(undefined4 *)(*(int *)(param_1[0x5c] + 0x68) + 0x24) = 1;
    (**(code **)(**(int **)(param_1[0x5c] + 0x68) + 0x38))();
    ppvVar1 = ExceptionList;
    goto LAB_0046881c;
  case 0x99:
    if (param_1[0x46] == 2) {
      ExceptionList = &pvStack_c;
      *(undefined4 *)(param_1[0x5c] + 0x6c) = 0;
      *(undefined4 *)(*(int *)(param_1[0x5c] + 0x68) + 0x24) = 1;
      *(undefined4 *)(param_1[0x5c] + 0x38) = 10;
      (**(code **)(**(int **)(param_1[0x5c] + 0x68) + 0x3c))();
      ppvVar1 = ExceptionList;
    }
    else {
      ppvVar1 = &pvStack_c;
      if (param_1[0x46] != 3) {
        return;
      }
    }
LAB_0046881c:
    ExceptionList = ppvVar1;
    (**(code **)(*param_1 + 0x48))(0x65,0,0);
LAB_00468829:
    iVar4 = param_1[0x5a];
    goto LAB_0046882f;
  case 0x9a:
    ExceptionList = &pvStack_c;
    *(undefined4 *)(*(int *)(param_1[0x5c] + 0x68) + 0x24) = 1;
    *(undefined4 *)(param_1[0x5c] + 0x38) = 10;
    iVar4 = (**(code **)(*(int *)param_1[0x5c] + 0x10))();
    if (iVar4 == 0) {
      ExceptionList = pvStack_c;
      return;
    }
    iVar4 = (**(code **)(*(int *)param_1[0x5c] + 0x10))();
    param_1[0x5c] = iVar4;
    FUN_00469de0(param_1,iVar4);
    iVar4 = param_1[0x77];
LAB_0046882f:
    SetFocus(*(HWND *)(iVar4 + 0x18));
  }
  ExceptionList = pvStack_c;
  return;
}

