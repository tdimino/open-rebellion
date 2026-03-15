
undefined4 FUN_00502bd0(void)

{
  bool bVar1;
  undefined4 uVar2;
  uint uVar3;
  undefined4 *puVar4;
  
  uVar2 = 1;
  if (DAT_006b2b20 == 0) {
    DAT_006b2b20 = 1;
    uVar3 = FUN_00557d80(0,&DAT_006b2b28);
    puVar4 = FUN_00520690(&DAT_006b2b28,0x203,1);
    if ((puVar4 == (undefined4 *)0x0) || (uVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b28,0x270,1);
    if ((puVar4 != (undefined4 *)0x0) && (bVar1)) {
      return 1;
    }
    uVar2 = 0;
  }
  return uVar2;
}

