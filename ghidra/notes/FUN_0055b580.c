
uint FUN_0055b580(void)

{
  uint uVar1;
  uint uVar2;
  undefined4 *puVar3;
  
  uVar1 = 1;
  if (DAT_006bb4d8 == 0) {
    DAT_006bb4d8 = 1;
    uVar2 = FUN_0052aeb0(0,&DAT_006bb4d0);
    uVar1 = 0;
    if (uVar2 != 0) {
      puVar3 = FUN_00520690(&DAT_006bb4d0,0x212,1);
      uVar1 = (uint)(puVar3 != (undefined4 *)0x0);
    }
    DAT_006bb4d8 = 1;
  }
  return uVar1;
}

