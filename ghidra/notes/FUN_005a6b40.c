
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005a6b40(void *this,int *param_1,float param_2)

{
  float fVar1;
  float fVar2;
  char cVar3;
  float *pfVar4;
  int iVar5;
  void *this_00;
  uint uVar6;
  int iVar7;
  int iStack_20;
  int iStack_1c;
  float fStack_18;
  undefined4 uStack_14;
  int local_10;
  float local_c;
  float local_8;
  int iStack_4;
  
  pfVar4 = (float *)FUN_005a7490(this,&local_10,param_1[0x5a]);
  local_c = -pfVar4[1];
  local_8 = -pfVar4[2];
  param_1[4] = (int)-*pfVar4;
  param_1[5] = (int)local_c;
  param_1[7] = -0x40800000;
  param_1[6] = (int)local_8;
  if ((float)param_1[0x14] == _DAT_0066c250) {
    iVar5 = (**(code **)(*param_1 + 0x20))();
    iVar7 = 1;
    if (iVar5 == 1) {
      uVar6 = 0x1d;
    }
    else {
      uVar6 = 0x1f;
    }
    this_00 = (void *)FUN_005ba8b0();
    FUN_005ba980(this_00,uVar6,iVar7);
  }
  fVar1 = param_2 + (float)param_1[0x14];
  param_1[0x14] = (int)fVar1;
  if (_DAT_0066c24c <= fVar1) {
    fVar1 = _DAT_0066c24c;
  }
  param_1[0x14] = (int)fVar1;
  fVar2 = fVar1 * _DAT_0066c26c;
  iVar5 = param_1[0x3b];
  iVar7 = (**(code **)(*param_1 + 0x24))();
  if ((iVar7 + iVar5 & 3U) == 0) {
    fVar2 = fVar2 * fVar2 * _DAT_006bc47c;
  }
  else {
    iVar5 = param_1[0x3b];
    cVar3 = (**(code **)(*param_1 + 0x24))();
    if ((cVar3 + (char)iVar5 & 3U) == 1) {
      fVar2 = fVar2 * fVar2 * _DAT_006bc488;
    }
    else {
      iVar5 = param_1[0x3b];
      cVar3 = (**(code **)(*param_1 + 0x24))();
      if ((cVar3 + (char)iVar5 & 3U) == 2) {
        fVar2 = fVar2 * fVar2 * _DAT_006bc484;
      }
      else {
        fVar2 = fVar2 * fVar2 * _DAT_006bc494;
      }
    }
  }
  iVar5 = FUN_005a7490(this,&local_10,param_1[0x5a]);
  iStack_20 = param_1[0xc];
  iStack_1c = param_1[0xd];
  fStack_18 = *(float *)(iVar5 + 8) * -fVar2 + (float)param_1[0xe];
  local_10 = param_1[4];
  local_c = (float)param_1[5];
  local_8 = (float)param_1[6];
  iStack_4 = param_1[7];
  uStack_14 = _DAT_0066c27c;
  FUN_005c12a0(param_1[0x3b],&iStack_20,&local_10);
  if (_DAT_0066c24c <= fVar1) {
    param_1[0x14] = 0;
    return 1;
  }
  return 0;
}

