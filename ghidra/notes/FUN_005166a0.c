
uint __cdecl FUN_005166a0(int *param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  undefined3 extraout_var;
  uint uVar5;
  
  puVar1 = param_2;
  *param_2 = 0;
  uVar3 = FUN_005185c0(param_1,(int *)&param_2);
  uVar5 = 0;
  if (uVar3 != 0) {
    uVar5 = (uint)(param_2 != (undefined4 *)0x0);
  }
  if (uVar5 != 0) {
    iVar4 = param_2[0x1b];
    if (iVar4 < 0) {
      iVar4 = -iVar4;
    }
    if (iVar4 != 0) {
      bVar2 = FUN_0053e2f0(iVar4);
      if (CONCAT31(extraout_var,bVar2) != 0) {
        *puVar1 = 1;
        return uVar5;
      }
    }
    *puVar1 = 0;
  }
  return uVar5;
}

