
int __thiscall FUN_00609650(void *this,int param_1,int param_2,int param_3)

{
  int *this_00;
  bool bVar1;
  int *piVar2;
  size_t sVar3;
  size_t sVar4;
  short sVar5;
  ushort uVar6;
  ushort uVar7;
  int *piVar8;
  int iVar9;
  uint uVar10;
  size_t local_10;
  int *piStack_c;
  
  local_10 = 0;
  piVar8 = (int *)(**(code **)(**(int **)((int)this + 0xa0) + 8))();
  sVar5 = FUN_005f3040(param_1);
  piVar2 = piVar8;
  sVar3 = local_10;
  if (sVar5 != 0) {
    while (piVar8 != (int *)0x0) {
      this_00 = piVar8 + 5;
      iVar9 = FUN_005f3430(this_00,param_1,1,param_3);
      local_10 = sVar3;
      piStack_c = piVar2;
      if (iVar9 == 0) {
        uVar6 = FUN_005f3040(param_1);
        uVar7 = FUN_005f3040((int)this_00);
        sVar4 = 1;
        do {
          local_10 = sVar4;
          uVar10 = (uint)uVar6;
          if ((uint)uVar7 <= (uint)uVar6) {
            uVar10 = (uint)uVar7;
          }
          if ((int)uVar10 <= (int)local_10) break;
          iVar9 = FUN_005f3430(this_00,param_1,local_10 + 1,param_3);
          sVar4 = local_10 + 1;
        } while (iVar9 == 0);
        piStack_c = piVar8;
        if (((int)local_10 <= (int)sVar3) &&
           (bVar1 = (int)local_10 < (int)sVar3, local_10 = sVar3, piStack_c = piVar2, bVar1)) break;
      }
      piVar8 = (int *)(**(code **)(*piVar8 + 0xc))();
      piVar2 = piStack_c;
      sVar3 = local_10;
    }
    if (piVar2 != (int *)0x0) {
      if ((*(byte *)(piVar2 + 0xf) & 1) == 0) {
        FUN_00609500(this,piVar2[3],param_2);
        FUN_00609ed0(this,piVar2[3]);
      }
      if (piVar2 != (int *)0x0) {
        return piVar2[3];
      }
    }
  }
  return 0;
}

