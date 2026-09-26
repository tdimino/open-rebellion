
void __thiscall FUN_0059b450(void *this,int param_1)

{
  void *this_00;
  int iVar1;
  int iVar2;
  uint *puVar3;
  undefined4 *puVar4;
  uint uVar5;
  int *piVar6;
  uint local_20 [8];
  
  FUN_00595ba0(local_20);
  uVar5 = 0;
  puVar3 = local_20;
  piVar6 = (int *)((int)this + 0xb8);
  while (((iVar1 = FUN_00602510(*piVar6), iVar1 != param_1 || (*puVar3 != 0)) ||
         (iVar1 = FUN_00595b90(), iVar1 != 0))) {
    uVar5 = uVar5 + 1;
    piVar6 = piVar6 + 1;
    puVar3 = puVar3 + 1;
    if (7 < uVar5) {
      puVar3 = local_20;
      puVar4 = (undefined4 *)((int)this + 0xd8);
      iVar1 = 2;
      do {
        this_00 = (void *)puVar4[-8];
        if ((*puVar3 != 0) && (this_00 != (void *)0x0)) {
          iVar2 = FUN_00602510((int)this_00);
          if (iVar2 == param_1) {
            FUN_00595b10(iVar1);
          }
          else {
            FUN_006030f0(this_00,5);
          }
        }
        iVar2 = FUN_00602510((int)this_00);
        if (((iVar2 == param_1) && (*puVar3 == 0)) && (iVar2 = FUN_00595b90(), iVar2 != 0)) {
          FUN_00600c40(this,this_00,*puVar4);
          FUN_005c5480(this_00);
          FUN_006030c0(this_00,5);
          FUN_00595b10(iVar1);
        }
        puVar3 = puVar3 + 1;
        puVar4 = puVar4 + 1;
        uVar5 = iVar1 - 1;
        iVar1 = iVar1 + 1;
      } while (uVar5 < 8);
      return;
    }
  }
  return;
}

