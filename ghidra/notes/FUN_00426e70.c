
undefined4 __thiscall
FUN_00426e70(void *this,int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            int param_7)

{
  undefined2 *puVar1;
  undefined2 uVar2;
  
  if (param_2 == 1) {
    puVar1 = (undefined2 *)((int)this + 0x294);
  }
  else if (param_2 == 2) {
    puVar1 = (undefined2 *)((int)this + 0x2a4);
  }
  else if (param_2 == 3) {
    puVar1 = (undefined2 *)((int)this + 0x2b4);
  }
  else {
    puVar1 = (undefined2 *)0x0;
  }
  if (puVar1 == (undefined2 *)0x0) {
    return 0;
  }
  uVar2 = (undefined2)((uint)puVar1 >> 0x10);
  if (param_3 <= param_1) {
    return CONCAT22(uVar2,puVar1[6]);
  }
  if ((param_1 <= param_4) && (param_5 <= param_1)) {
    return CONCAT22(uVar2,puVar1[4]);
  }
  if ((param_1 <= param_6) && (param_7 <= param_1)) {
    return CONCAT22(uVar2,puVar1[2]);
  }
  return CONCAT22(uVar2,*puVar1);
}

