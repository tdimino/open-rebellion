
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005aefc0(void *this,int param_1)

{
  int *piVar1;
  int iVar2;
  int *piVar3;
  undefined4 *puVar4;
  uint uVar5;
  uint uVar6;
  float10 fVar7;
  longlong lVar8;
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652fa5;
  pvStack_c = ExceptionList;
  if (param_1 == 0) {
    piVar1 = (int *)0x0;
  }
  else {
    piVar1 = (int *)(param_1 + 0x1c);
  }
  ExceptionList = &pvStack_c;
  FUN_005c6100(local_18,piVar1);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  uVar6 = 0;
  piVar1 = (int *)thunk_FUN_005f5060((int)local_18);
  iVar2 = thunk_FUN_005f5060((int)local_18);
  while( true ) {
    if (iVar2 == 0) goto LAB_005af3f7;
    iVar2 = (**(code **)(*piVar1 + 8))();
    if (iVar2 == 0) {
      piVar3 = (int *)thunk_FUN_005f5060((int)local_18);
    }
    else {
      piVar3 = (int *)(**(code **)(*piVar1 + 8))();
    }
    uVar6 = uVar6 + 1;
    if (100 < uVar6) break;
    piVar1 = (int *)(**(code **)(*piVar1 + 0x10))();
    (**(code **)(*piVar1 + 0xbc))();
    lVar8 = __ftol();
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      puVar4 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 1;
      if (puVar4 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar4);
      }
      local_4._0_1_ = 0;
    }
    uVar5 = FUN_005af420(DAT_006bc4a8,1,10);
    if ((uint)lVar8 < uVar5) {
      lVar8 = __ftol();
      uVar5 = (uint)lVar8;
      if (uVar5 == 0) {
        FUN_005af7a0(this,piVar1);
        uVar5 = piVar1[0x3b];
      }
      else if (uVar5 < 2) {
        FUN_005af7a0(this,piVar1);
        uVar5 = piVar1[0x3b];
      }
      else {
        if (DAT_006bc4a8 == (undefined4 *)0x0) {
          puVar4 = (undefined4 *)FUN_00618b70(0x274);
          local_4._0_1_ = 2;
          if (puVar4 == (undefined4 *)0x0) {
            DAT_006bc4a8 = (undefined4 *)0x0;
          }
          else {
            DAT_006bc4a8 = FUN_005af5f0(puVar4);
          }
          local_4._0_1_ = 0;
        }
        uVar5 = FUN_005af420(DAT_006bc4a8,1,uVar5);
        FUN_005b54d0(piVar1,(float)uVar5 * _DAT_0066c3dc);
        if (_DAT_0066c3e8 < (float)piVar1[0xf3]) goto LAB_005af1a1;
        FUN_005af7a0(this,piVar1);
        uVar5 = piVar1[0x3b];
      }
      FUN_005c60d0(local_18,uVar5);
      iVar2 = thunk_FUN_005f5060((int)local_18);
      if (iVar2 == 0) goto LAB_005af3f7;
    }
    else {
LAB_005af1a1:
      if (_DAT_0066c3e8 < (float)piVar1[0xf3]) {
        if (DAT_006bc4a8 == (undefined4 *)0x0) {
          puVar4 = (undefined4 *)FUN_00618b70(0x274);
          local_4._0_1_ = 3;
          if (puVar4 == (undefined4 *)0x0) {
            DAT_006bc4a8 = (undefined4 *)0x0;
          }
          else {
            DAT_006bc4a8 = FUN_005af5f0(puVar4);
          }
          local_4._0_1_ = 0;
        }
        fVar7 = FUN_005afc00(DAT_006bc4a8,0);
        if (fVar7 < (float10)_DAT_0066c3ec) {
          if (DAT_006bc4a8 == (undefined4 *)0x0) {
            puVar4 = (undefined4 *)FUN_00618b70(0x274);
            local_4._0_1_ = 4;
            if (puVar4 == (undefined4 *)0x0) {
              DAT_006bc4a8 = (undefined4 *)0x0;
            }
            else {
              DAT_006bc4a8 = FUN_005af5f0(puVar4);
            }
            local_4._0_1_ = 0;
          }
          FUN_005afc00(DAT_006bc4a8,0);
        }
        FUN_005b23a0(piVar1);
        if (DAT_006bc4a8 == (undefined4 *)0x0) {
          puVar4 = (undefined4 *)FUN_00618b70(0x274);
          local_4._0_1_ = 5;
          if (puVar4 == (undefined4 *)0x0) {
            DAT_006bc4a8 = (undefined4 *)0x0;
          }
          else {
            DAT_006bc4a8 = FUN_005af5f0(puVar4);
          }
          local_4._0_1_ = 0;
        }
        uVar5 = FUN_005af420(DAT_006bc4a8,1,100);
        lVar8 = __ftol();
        if (uVar5 <= (uint)lVar8) {
          if (DAT_006bc4a8 == (undefined4 *)0x0) {
            puVar4 = (undefined4 *)FUN_00618b70(0x274);
            local_4._0_1_ = 6;
            if (puVar4 == (undefined4 *)0x0) {
              DAT_006bc4a8 = (undefined4 *)0x0;
            }
            else {
              DAT_006bc4a8 = FUN_005af5f0(puVar4);
            }
            local_4._0_1_ = 0;
          }
          if (DAT_006bc470 == (undefined4 *)0x0) {
            puVar4 = (undefined4 *)FUN_00618b70(0x48);
            local_4._0_1_ = 7;
            if (puVar4 == (undefined4 *)0x0) {
              DAT_006bc470 = (undefined4 *)0x0;
            }
            else {
              DAT_006bc470 = FUN_005a6290(puVar4);
            }
            local_4._0_1_ = 0;
          }
          iVar2 = FUN_005a66a0((int)DAT_006bc470);
          if (iVar2 != 0) {
            FUN_005ba8a0(iVar2);
            piVar1 = (int *)(**(code **)(**(int **)(iVar2 + 0x38) + 0x10))();
            while (piVar1 != (int *)0x0) {
              piVar3 = (int *)(**(code **)(*piVar1 + 8))();
              piVar1 = (int *)(**(code **)(*piVar1 + 0x10))();
              FUN_005af7a0(this,piVar1);
              piVar1 = piVar3;
            }
            (**(code **)(**(int **)(iVar2 + 0x38) + 4))();
            piVar1 = (int *)(**(code **)(**(int **)(iVar2 + 0x3c) + 0x10))();
            while (piVar1 != (int *)0x0) {
              piVar3 = (int *)(**(code **)(*piVar1 + 8))();
              piVar1 = (int *)(**(code **)(*piVar1 + 0x10))();
              FUN_005af7a0(this,piVar1);
              piVar1 = piVar3;
            }
            (**(code **)(**(int **)(iVar2 + 0x3c) + 4))();
          }
          goto LAB_005af3f7;
        }
      }
    }
    iVar2 = thunk_FUN_005f5060((int)local_18);
    piVar1 = piVar3;
  }
  piVar1 = (int *)thunk_FUN_005f5060((int)local_18);
  while (piVar1 != (int *)0x0) {
    piVar3 = (int *)(**(code **)(*piVar1 + 8))();
    piVar1 = (int *)(**(code **)(*piVar1 + 0x10))();
    FUN_005af7a0(this,piVar1);
    piVar1 = piVar3;
  }
LAB_005af3f7:
  local_4 = 0xffffffff;
  FUN_005c5ff0(local_18);
  ExceptionList = pvStack_c;
  return;
}

