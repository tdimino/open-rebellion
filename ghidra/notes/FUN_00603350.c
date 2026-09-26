
void __thiscall FUN_00603350(void *this,undefined4 param_1,int param_2)

{
  undefined4 *puVar1;
  bool bVar2;
  undefined3 extraout_var;
  
  if ((param_2 != 0) || (bVar2 = FUN_005fc040((int *)0x0), CONCAT31(extraout_var,bVar2) != 0)) {
    switch(param_1) {
    case 0:
      puVar1 = *(undefined4 **)((int)this + 0x94);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
      }
      *(int *)((int)this + 0x94) = param_2;
      *(undefined2 *)((int)this + 0xac) = 0;
      return;
    case 1:
      puVar1 = *(undefined4 **)((int)this + 0x98);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
      }
      *(int *)((int)this + 0x98) = param_2;
      *(undefined2 *)((int)this + 0xae) = 0;
      return;
    case 2:
    case 0x40:
      puVar1 = *(undefined4 **)((int)this + 0x9c);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
      }
      *(int *)((int)this + 0x9c) = param_2;
      *(undefined2 *)((int)this + 0xb0) = 0;
      break;
    case 4:
      puVar1 = *(undefined4 **)((int)this + 0xa0);
      if (puVar1 != (undefined4 *)0x0) {
        FUN_005fbfa0(puVar1);
        FUN_00618b60((undefined *)puVar1);
      }
      *(int *)((int)this + 0xa0) = param_2;
      *(undefined2 *)((int)this + 0xb2) = 0;
      return;
    }
  }
  return;
}

