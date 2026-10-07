
uint FUN_0055bd30(void)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006bb4f8 == 0) {
    DAT_006bb4f8 = 1;
    uVar2 = FUN_0052aeb0(0,&DAT_006bb500);
    uVar1 = 0;
    if (uVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006bb500,0x215,1);
      uVar1 = 0;
      if (puVar3 != (undefined4 *)0x0) {
        puVar3 = FUN_00520690(&DAT_006bb500,0x211,1);
        uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
      }
    }
    DAT_006bb4f8 = 1;
  }
  return uVar1;
}

