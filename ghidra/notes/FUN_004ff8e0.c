
undefined4 FUN_004ff8e0(void)

{
  bool bVar1;
  undefined4 uVar2;
  int iVar3;
  undefined4 *puVar4;
  
  uVar2 = 1;
  if (DAT_006b2b18 == 0) {
    DAT_006b2b18 = 1;
    iVar3 = FUN_004fad70(0,&DAT_006b2b10);
    puVar4 = FUN_00520690(&DAT_006b2b10,0x201,1);
    if ((puVar4 == (undefined4 *)0x0) || (iVar3 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x202,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x204,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x203,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x220,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x221,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x222,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x223,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x234,1);
    if ((puVar4 == (undefined4 *)0x0) || (!bVar1)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puVar4 = FUN_00520690(&DAT_006b2b10,0x200,1);
    if ((puVar4 != (undefined4 *)0x0) && (bVar1)) {
      return 1;
    }
    uVar2 = 0;
  }
  return uVar2;
}

