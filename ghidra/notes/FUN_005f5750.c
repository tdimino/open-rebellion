
int FUN_005f5750(void)

{
  DWORD DVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  
  DVar1 = FUN_0061b120((int *)0x0);
  FUN_0061a300(DVar1);
  do {
    uVar2 = FUN_0061a310();
    uVar3 = FUN_0061a310();
    uVar4 = FUN_0061a310();
    uVar5 = FUN_0061a310();
    iVar6 = uVar2 * uVar3 + uVar4 * uVar5;
  } while (iVar6 == 0x7fffffff);
  return iVar6;
}

