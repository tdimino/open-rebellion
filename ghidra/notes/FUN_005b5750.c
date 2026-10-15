
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005b5750(int *param_1,float param_2,uint param_3,uint param_4)

{
  float fVar1;
  float fVar2;
  bool bVar3;
  void *pvVar4;
  int *piVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  
  fVar1 = (float)param_1[0xf3];
  fVar2 = (float)param_1[0xf0];
  if (fVar1 <= _DAT_0066c4f0) {
    param_1[0xf3] = 0;
    return;
  }
  if (param_2 < _DAT_0066c514) {
    param_2 = 1.0;
  }
  param_2 = (float)param_1[0xf0] - param_2;
  bVar3 = _DAT_0066c4f0 <= param_2;
  param_1[0xf0] = (int)param_2;
  if (bVar3) {
    iVar7 = 1;
    uVar6 = 0xc;
    pvVar4 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar4,uVar6,iVar7);
    if ((param_1[0x49] == 0) && (iVar7 = (**(code **)(*param_1 + 0x20))(), iVar7 == 0)) {
      param_2 = (_DAT_0066c51c / fVar2) * (fVar2 - (float)param_1[0xf0]);
      if (param_2 < _DAT_0066c514) {
        param_2 = 1.0;
      }
      iVar8 = 100;
      iVar7 = 0;
      FUN_005aaf90();
      uVar6 = FUN_005a8a70(iVar7,iVar8);
      if ((float)uVar6 <= param_2) {
        piVar5 = (int *)FUN_005aaf90();
        (**(code **)(*piVar5 + 0x28))(0x1e,param_3,0,1);
      }
    }
    uVar6 = param_1[0x3b];
    iVar7 = (**(code **)(*param_1 + 0xb4))();
    FUN_005c13d0(uVar6,3,iVar7);
  }
  else {
    iVar7 = 1;
    uVar6 = 0x11;
    param_1[0xf0] = 0;
    param_1[0xf3] = (int)(param_2 + (float)param_1[0xf3]);
    pvVar4 = (void *)FUN_005ba8b0();
    FUN_005ba980(pvVar4,uVar6,iVar7);
    if ((float)param_1[0xf3] <= _DAT_0066c4f0) {
      pvVar4 = (void *)FUN_0059ed70();
      FUN_005a5cf0(pvVar4,param_3,param_4);
      param_1[0xf3] = 0;
      uVar6 = param_1[0x3b];
      iVar7 = (**(code **)(*param_1 + 0xb8))();
      FUN_005c13d0(uVar6,3,iVar7);
      param_1[0x3d] = 0;
      param_1[0x40] = 0;
      param_1[0x44] = 0;
      (**(code **)(*param_1 + 0xd0))();
      (**(code **)(*param_1 + 0x1c))();
      return;
    }
    uVar6 = param_1[0x3b];
    iVar7 = (**(code **)(*param_1 + 0xb0))();
    FUN_005c13d0(uVar6,3,iVar7);
    if ((param_1[0x49] == 0) && (iVar7 = (**(code **)(*param_1 + 0x20))(), iVar7 != 1)) {
      iVar8 = 100;
      iVar7 = 0;
      FUN_005aaf90();
      uVar6 = FUN_005a8a70(iVar7,iVar8);
      fVar1 = (((float)param_1[0xf3] - fVar1) * _DAT_0066c51c) / fVar1 + (float)uVar6;
      if (_DAT_0066c520 < fVar1) {
        if (fVar1 <= _DAT_0066c524) {
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x28))(0x1e,param_3,0,1);
          return;
        }
        if (fVar1 <= _DAT_0066c528) {
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x28))(0x1f,param_3,0,1);
          return;
        }
        if (fVar1 <= _DAT_0066c52c) {
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x28))(0x20,param_3,0,1);
          return;
        }
        if (_DAT_0066c530 < fVar1) {
          if (_DAT_0066c51c < fVar1) {
            param_1[0xf3] = 0;
            piVar5 = (int *)FUN_005aaf90();
            (**(code **)(*piVar5 + 0x28))(3,param_3,param_4,1);
            return;
          }
          piVar5 = (int *)FUN_005aaf90();
          (**(code **)(*piVar5 + 0x28))(0x22,param_3,0,1);
          return;
        }
        piVar5 = (int *)FUN_005aaf90();
        (**(code **)(*piVar5 + 0x28))(0x21,param_3,0,1);
        return;
      }
    }
  }
  return;
}

