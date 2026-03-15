
void __fastcall FUN_00410e90(int *param_1)

{
  int iVar1;
  uint uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int *piVar5;
  int iVar6;
  int *piStack_44;
  undefined4 auStack_40 [3];
  undefined4 auStack_34 [9];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062a5cf;
  pvStack_c = ExceptionList;
  DAT_006b123c = 0;
  iVar6 = 0;
  ExceptionList = &pvStack_c;
  KillTimer((HWND)param_1[6],0x191);
  iVar1 = FUN_00597480();
  if (iVar1 == 0) {
    if (param_1[0x47] == 0) {
      FUN_0040f790((int)param_1);
      iVar6 = FUN_00597240(param_1[100]);
      if (param_1[0x5e] != 0) {
        piStack_44 = (int *)(param_1[0x5e] + 0x98);
      }
    }
    else {
      piVar5 = (int *)(param_1[0x5c] + 0x98);
      if (param_1[0x5c] == 0) {
        piVar5 = piStack_44;
      }
      uVar2 = FUN_005f3070((int)piVar5);
      if ((short)uVar2 == 0) {
        pvVar3 = FUN_00411340(param_1,auStack_40);
        uStack_4 = 0;
        FUN_005f3090(piVar5,(int)pvVar3);
        uStack_4 = 0xffffffff;
        FUN_005f2ff0(auStack_40);
      }
      iVar1 = FUN_00583c40((int)piVar5);
      iVar6 = FUN_005971e0(iVar1);
      if (param_1[0x5d] != 0) {
        piStack_44 = (int *)(param_1[0x5d] + 0x98);
      }
    }
    FUN_005f2f90(&stack0xffffffa0,(int)piStack_44);
    FUN_005972a0();
  }
  if (iVar6 == 0) {
    if (param_1[0x47] == 0) {
      FUN_0040fbb0(param_1,0);
      FUN_005f35b0(auStack_40,&DAT_006a816c);
      uStack_4 = 1;
      FUN_00410240(param_1,(int)auStack_40);
      uStack_4 = 0xffffffff;
      FUN_005f2ff0(auStack_40);
      puVar4 = (undefined4 *)FUN_00618b70(0x3c);
      if (puVar4 == (undefined4 *)0x0) {
        FUN_00619730();
      }
      uStack_4 = 2;
      if (puVar4 == (undefined4 *)0x0) {
        piStack_44 = (int *)0x0;
      }
      else {
        piStack_44 = FUN_00420bd0(puVar4);
      }
      uStack_4 = 0xffffffff;
      if (piStack_44 != (int *)0x0) {
        iVar1 = 1;
        piVar5 = piStack_44 + 9;
        do {
          iVar6 = 0;
          if (iVar1 < 7) {
            FUN_00401860(auStack_34,iVar1,1);
            iVar6 = iStack_10;
            FUN_004118f0(auStack_34);
          }
          *piVar5 = iVar6;
          iVar1 = iVar1 + 1;
          piVar5 = piVar5 + 1;
        } while (iVar1 < 6);
        FUN_004106f0(piStack_44);
      }
    }
    else {
      FUN_0040fbb0(param_1,1);
      FUN_004109e0((int)param_1);
    }
    FUN_0040f850((int)param_1);
    param_1[0x4b] = 0x2777;
    FUN_006075e0(param_1,0x2777);
    if (((((param_1[0x4f] != 0) && (param_1[0x50] != 0)) && (param_1[0x51] != 0)) &&
        (((param_1[0x52] != 0 && (param_1[0x53] != 0)) &&
         ((param_1[0x54] != 0 && ((param_1[0x55] != 0 && (param_1[0x56] != 0)))))))) &&
       ((param_1[0x57] != 0 &&
        ((((param_1[0x58] != 0 && (param_1[0x59] != 0)) && (param_1[0x5a] != 0)) &&
         (param_1[0x5b] != 0)))))) {
      iVar1 = FUN_00597480();
      if (iVar1 == 0) {
        (**(code **)(*(int *)param_1[0x4f] + 0xc))();
      }
      else {
        (**(code **)(*(int *)param_1[0x4f] + 0x10))();
      }
      (**(code **)(*(int *)param_1[0x51] + 4))();
      (**(code **)(*(int *)param_1[0x54] + 4))();
      (**(code **)(*(int *)param_1[0x55] + 4))();
      (**(code **)(*(int *)param_1[0x56] + 4))(5);
      (**(code **)(*(int *)param_1[0x57] + 4))(5);
      (**(code **)(*(int *)param_1[0x58] + 4))(5);
      (**(code **)(*(int *)param_1[0x59] + 4))(5);
      (**(code **)(*(int *)param_1[0x5a] + 4))(5);
      (**(code **)(*(int *)param_1[0x5b] + 4))(5);
    }
    if ((int *)param_1[0x5f] != (int *)0x0) {
      (**(code **)(*(int *)param_1[0x5f] + 4))();
      SetFocus(*(HWND *)(param_1[0x5f] + 0x18));
    }
    if ((void *)param_1[0x60] != (void *)0x0) {
      FUN_00420ba0((void *)param_1[0x60]);
      (**(code **)(*(int *)param_1[0x60] + 4))();
    }
    SetTimer((HWND)param_1[6],400,500,(TIMERPROC)0x0);
    ExceptionList = pvStack_c;
    return;
  }
  if (param_1[0x47] == 0) {
    FUN_00410c70(param_1);
    ExceptionList = pvStack_c;
    return;
  }
  FUN_00410dd0(param_1);
  ExceptionList = pvStack_c;
  return;
}

