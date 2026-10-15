
uint FUN_00539c20(void)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006b9040 == 0) {
    DAT_006b9040 = 1;
    iVar2 = FUN_004f2a10(0,&DAT_006b9038);
    uVar1 = 0;
    if (iVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006b9038,0x200,1);
      uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
    }
    DAT_006b9040 = 1;
  }
  return uVar1;
}

