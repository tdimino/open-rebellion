
uint __cdecl FUN_0053e1d0(int param_1)

{
  uint uVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  bool bVar7;
  
  if (3 < param_1) {
    uVar3 = 2;
    if (4 < param_1) {
      do {
        uVar3 = uVar3 * 2;
      } while ((int)(uVar3 * uVar3) < param_1);
    }
    uVar6 = (int)uVar3 / 2;
    uVar4 = uVar3;
    do {
      uVar1 = (int)(uVar6 + uVar4) / 2;
      iVar2 = uVar1 * uVar1;
      uVar5 = uVar1;
      if ((iVar2 - param_1 == 0 || iVar2 < param_1) && (uVar5 = uVar4, iVar2 < param_1)) {
        uVar6 = uVar1;
      }
      bVar7 = uVar1 != uVar3;
      uVar3 = uVar1;
      uVar4 = uVar5;
    } while (bVar7);
    return uVar1;
  }
  return (uint)(0 < param_1);
}

