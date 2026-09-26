
bool __cdecl FUN_00626ef0(int *param_1)

{
  uint uVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  
  if (DAT_006b01b4 == 0) {
    return false;
  }
  uVar7 = param_1[5];
  if ((uVar7 == DAT_006b0248) && (uVar7 == DAT_006b0258)) goto LAB_006270c4;
  if (DAT_006bec40 == 0) {
    FUN_00627160(1,1,uVar7,4,1,0,0,2,0,0,0);
    uVar7 = param_1[5];
    uVar11 = 0;
    uVar3 = 0;
    uVar10 = 0;
    uVar4 = 2;
    uVar1 = 0;
    uVar9 = 5;
    uVar8 = 10;
LAB_006270b8:
    uVar5 = 0;
    iVar6 = 1;
  }
  else {
    if (DAT_006bece0 != 0) {
      uVar10 = (uint)DAT_006bece4._2_2_;
      uVar3 = 0;
      uVar1 = 0;
    }
    else {
      uVar3 = DAT_006bece4 & 0xffff;
      uVar10 = 0;
      uVar1 = (uint)DAT_006bece4._2_2_;
    }
    FUN_00627160(1,(uint)(DAT_006bece0 == 0),uVar7,(uint)DAT_006bece2,uVar1,uVar3,uVar10,
                 DAT_006bece8 & 0xffff,DAT_006bece8 >> 0x10,DAT_006becec & 0xffff,
                 DAT_006becec >> 0x10);
    if (DAT_006bec8c == 0) {
      uVar11 = (uint)DAT_006bec98._2_2_;
      uVar3 = DAT_006bec98 & 0xffff;
      uVar10 = (uint)DAT_006bec94._2_2_;
      uVar4 = DAT_006bec94 & 0xffff;
      uVar1 = DAT_006bec90 & 0xffff;
      uVar9 = (uint)DAT_006bec90._2_2_;
      uVar8 = (uint)DAT_006bec8e;
      uVar7 = param_1[5];
      goto LAB_006270b8;
    }
    uVar11 = (uint)DAT_006bec98._2_2_;
    uVar3 = DAT_006bec98 & 0xffff;
    uVar10 = (uint)DAT_006bec94._2_2_;
    uVar5 = (uint)DAT_006bec90._2_2_;
    uVar4 = DAT_006bec94 & 0xffff;
    uVar7 = param_1[5];
    uVar8 = (uint)DAT_006bec8e;
    uVar1 = 0;
    uVar9 = 0;
    iVar6 = 0;
  }
  FUN_00627160(0,iVar6,uVar7,uVar8,uVar9,uVar1,uVar5,uVar4,uVar10,uVar3,uVar11);
LAB_006270c4:
  iVar6 = param_1[7];
  if (DAT_006b024c < DAT_006b025c) {
    if ((iVar6 < DAT_006b024c) || (DAT_006b025c < iVar6)) {
      return false;
    }
    if ((DAT_006b024c < iVar6) && (iVar6 < DAT_006b025c)) {
      return true;
    }
  }
  else {
    if ((iVar6 < DAT_006b025c) || (DAT_006b024c < iVar6)) {
      return true;
    }
    if ((DAT_006b025c < iVar6) && (iVar6 < DAT_006b024c)) {
      return false;
    }
  }
  iVar2 = (*param_1 + (param_1[1] + param_1[2] * 0x3c) * 0x3c) * 1000;
  if (iVar6 != DAT_006b024c) {
    return iVar2 < DAT_006b0260;
  }
  return DAT_006b0250 <= iVar2;
}

