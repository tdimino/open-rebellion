
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __fastcall FUN_005d04e0(int param_1)

{
  bool bVar1;
  bool bVar2;
  undefined4 uVar3;
  int *piVar4;
  uint uVar5;
  int *piVar6;
  void *pvVar7;
  float10 fVar8;
  longlong lVar9;
  int iVar10;
  int iVar11;
  uint local_2c;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006544e8;
  local_c = ExceptionList;
  iVar10 = *(int *)(param_1 + 8);
  if (*(int *)(iVar10 + 0x34) == 0) {
    if (iVar10 == 0) {
      piVar4 = (int *)0x0;
    }
    else {
      piVar4 = (int *)(iVar10 + 0x1c);
    }
    ExceptionList = &local_c;
    FUN_005c6100(local_18,piVar4);
    local_4 = 0;
    bVar2 = false;
    *(undefined4 *)(param_1 + 0x5c) = 0x11e;
    *(undefined4 *)(param_1 + 0x60) = 0x11f;
    *(undefined4 *)(param_1 + 100) = 0x120;
    *(undefined4 *)(param_1 + 0x68) = 0x128;
    *(undefined4 *)(param_1 + 0x6c) = 0x121;
    *(undefined4 *)(param_1 + 0x70) = 0x122;
    *(undefined4 *)(param_1 + 0x74) = 0x129;
    *(undefined4 *)(param_1 + 0x78) = 0x13d;
    *(undefined4 *)(param_1 + 0x7c) = 0x13d;
    local_2c = 0;
    piVar4 = (int *)thunk_FUN_005f5060((int)local_18);
joined_r0x005d0576:
    if (piVar4 != (int *)0x0) {
      uVar5 = (**(code **)(*piVar4 + 4))();
      FUN_005f54d0(local_18,uVar5);
      piVar6 = (int *)(**(code **)(*piVar4 + 0x10))();
      if (piVar4 != (int *)0x0) {
        (**(code **)*piVar4)(1);
      }
      bVar1 = false;
      (**(code **)(*piVar6 + 0xbc))();
      lVar9 = __ftol();
      iVar11 = 10;
      iVar10 = 1;
      FUN_005aaf90();
      uVar5 = FUN_005a8a70(iVar10,iVar11);
      if ((uint)lVar9 < uVar5) {
        bVar2 = true;
        lVar9 = __ftol();
        uVar5 = (uint)lVar9;
        if (uVar5 == 0) {
          (**(code **)(*piVar6 + 0xc0))(0x47c35000);
        }
        else {
          if (uVar5 < 2) {
            (**(code **)(*piVar6 + 0xc0))(0x47c35000);
          }
          else {
            iVar10 = 1;
            FUN_005aaf90();
            uVar5 = FUN_005a8a70(iVar10,uVar5);
            (**(code **)(*piVar6 + 0xc0))((float)uVar5 * _DAT_0066d078);
          }
          if ((float)piVar6[0xf3] <= _DAT_0066d090) {
            (**(code **)(*piVar6 + 0xc0))(0x47c35000);
          }
        }
      }
      if (_DAT_0066d090 < (float)piVar6[0xf3]) {
        iVar10 = 0;
        bVar1 = true;
        pvVar7 = (void *)FUN_005aaf90();
        fVar8 = FUN_005ad7e0(pvVar7,iVar10);
        if (fVar8 < (float10)_DAT_0066d0a0) {
          iVar10 = 0;
          pvVar7 = (void *)FUN_005aaf90();
          FUN_005ad7e0(pvVar7,iVar10);
        }
        FUN_005b23a0(piVar6);
        iVar11 = 100;
        iVar10 = 1;
        FUN_005aaf90();
        uVar5 = FUN_005a8a70(iVar10,iVar11);
        lVar9 = __ftol();
        if ((uint)lVar9 < uVar5) goto LAB_005d06e9;
        if (bVar2) {
          *(undefined4 *)(param_1 + 0x60) = 0x123;
          *(undefined4 *)(param_1 + 100) = 0x124;
          *(undefined4 *)(param_1 + 0x68) = 0x125;
          *(undefined4 *)(param_1 + 0x6c) = 0x12a;
          *(undefined4 *)(param_1 + 0x70) = 299;
          *(undefined4 *)(param_1 + 0x74) = 0x126;
        }
        *(undefined4 *)(param_1 + 0x34) = 0;
        *(undefined4 *)(param_1 + 0x2c) = 9;
        *(undefined4 *)(param_1 + 0x30) = DAT_0066d080;
        for (piVar4 = (int *)(**(code **)(*(int *)(*(int *)(param_1 + 8) + 0x1c) + 0x10))();
            piVar4 != (int *)0x0; piVar4 = (int *)(**(code **)(*piVar4 + 8))()) {
          iVar10 = (**(code **)(*piVar4 + 0x10))();
          *(undefined4 *)(iVar10 + 0x3cc) = *(undefined4 *)(iVar10 + 0x3d0);
          *(undefined4 *)(iVar10 + 0x3c0) = *(undefined4 *)(iVar10 + 0x3c4);
        }
        goto LAB_005d076a;
      }
LAB_005d06e9:
      local_2c = local_2c + 1;
      if (local_2c < 0x3e9) goto code_r0x005d06f9;
    }
    *(undefined4 *)(param_1 + 0x60) = 0x123;
    *(undefined4 *)(param_1 + 100) = 0x124;
    *(undefined4 *)(param_1 + 0x68) = 0x125;
    *(undefined4 *)(param_1 + 0x6c) = 0x12a;
    *(undefined4 *)(param_1 + 0x70) = 299;
    *(undefined4 *)(param_1 + 0x74) = 0x126;
    *(undefined4 *)(param_1 + 0x78) = 300;
    *(undefined4 *)(param_1 + 0x7c) = 0x127;
    uVar3 = DAT_0066d084;
    *(undefined4 *)(param_1 + 0x34) = 0;
    *(undefined4 *)(param_1 + 0x30) = uVar3;
    *(undefined4 *)(param_1 + 0x2c) = 10;
LAB_005d076a:
    local_4 = 0xffffffff;
    FUN_005c5ff0(local_18);
  }
  ExceptionList = local_c;
  return;
code_r0x005d06f9:
  if (bVar1) {
    FUN_005c5f20(local_18,(int)piVar6);
  }
  piVar4 = (int *)thunk_FUN_005f5060((int)local_18);
  goto joined_r0x005d0576;
}

