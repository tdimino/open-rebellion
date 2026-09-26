
void __thiscall FUN_005d3e90(int *param_1,void *param_2,int param_3)

{
  int *piVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  void *pvVar5;
  int iVar6;
  void *unaff_ESI;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006546fb;
  pvStack_c = ExceptionList;
  if (param_3 == 0) {
    uVar3 = (-(uint)(param_2 != (void *)0x2) & 4) + 4;
  }
  else if (param_3 == 1) {
    if (param_2 == (void *)0x2) {
      uVar3 = 0x10;
    }
    else if (param_2 == (void *)0x3) {
      uVar3 = 0x40;
    }
    else {
      uVar3 = 0x20;
    }
  }
  else if (param_3 == 2) {
    uVar3 = 0x80;
  }
  else {
    uVar3 = 0;
  }
  uVar2 = param_1[0x48];
  if (uVar3 <= uVar2) {
    return;
  }
  if (((uVar3 & 0x70) != 0) && ((uVar2 & 0x70) != 0)) {
    return;
  }
  if (((uVar3 & 0xc) != 0) && ((uVar2 & 0xc) != 0)) {
    return;
  }
  ExceptionList = &pvStack_c;
  param_1[0x48] = uVar2 & 0xffffff03 | uVar3;
  switch(uVar3) {
  case 4:
    param_1[0x51] = 0xc6c;
    param_1[0x4f] = 6;
    break;
  case 8:
    param_1[0x51] = 0xbf4;
    param_1[0x4f] = 6;
    break;
  case 0x10:
    param_1[0x51] = 0xca8;
    param_1[0x4f] = 0x10;
    break;
  case 0x20:
    param_1[0x51] = 0xc30;
    goto LAB_005d3fa8;
  case 0x40:
    param_1[0x51] = 0xce4;
LAB_005d3fa8:
    param_1[0x4f] = 7;
    break;
  case 0x80:
    param_1[0x51] = 0xd20;
    param_1[0x4f] = 0x10;
  }
  if (param_1[0x52] != 0) {
    iVar4 = (**(code **)(*(int *)param_1[0x4a] + 0xa4))((int *)param_1[0x4a],param_1[0x52],0);
    FUN_005d8b00(iVar4);
    param_1[0x52] = 0;
  }
  param_2 = (void *)FUN_00618b70(0x14);
  uStack_4 = 0;
  if (param_2 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_005ee4f0(param_2,(int)param_1);
  }
  uStack_4 = 0xffffffff;
  uVar3 = FUN_005f58d0((void *)(DAT_006bcbd4 + 0x5e4),(int)pvVar5);
  param_1[0x50] = 0;
  param_2 = (void *)CONCAT22((short)(uVar3 >> 0x10),(short)param_1[0x51] + (short)param_1[0x50]);
  piVar1 = (int *)(DAT_006bcbd4 + 0x168);
  for (iVar4 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10)) {
    if (*(short *)(iVar4 + 0x18) == (short)param_2) goto LAB_005d405b;
  }
  iVar4 = 0;
LAB_005d405b:
  if (iVar4 == 0) {
    iVar4 = (**(code **)(*piVar1 + 4))(&param_2);
    param_1[0x52] = iVar4;
  }
  else {
    param_1[0x52] = *(int *)(iVar4 + 0x1c);
  }
  iVar4 = param_1[6];
  iVar6 = FUN_005c2590(DAT_006bcbd4);
  if (iVar6 == iVar4) {
    (**(code **)(*param_1 + 0x24))(1,0);
    (**(code **)(*param_1 + 0x24))(2,0);
    param_1[0x52] = 0;
    ExceptionList = pvStack_c;
    return;
  }
  iVar4 = (**(code **)(*(int *)param_1[0x4a] + 0x48))((int *)param_1[0x4a],param_1[0x52],0);
  FUN_005d8b00(iVar4);
  ExceptionList = unaff_ESI;
  return;
}

