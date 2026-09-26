
undefined4 __thiscall
FUN_005fa2b0(void *this,undefined4 *param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4
            ,undefined4 param_5,int param_6,int param_7)

{
  undefined4 uVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined4 local_4;
  
  local_4 = 0;
  if (param_1 != (undefined4 *)0x0) {
    puVar3 = param_1;
    for (iVar2 = 0x14; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
    *param_1 = 0x50;
    param_1[10] = DAT_006ac294;
    param_1[1] = param_6;
    param_1[6] = *(undefined4 *)((int)this + 0xb0);
    param_1[7] = *(undefined4 *)((int)this + 0xb4);
    param_1[8] = *(undefined4 *)((int)this + 0xb8);
    uVar1 = *(undefined4 *)((int)this + 0xbc);
    param_1[0xc] = param_7;
    param_1[9] = uVar1;
    param_1[2] = param_2;
    param_1[3] = param_3;
    param_1[4] = param_4;
    param_1[5] = param_5;
  }
  if ((param_6 == 2) && (param_7 == 0)) {
    local_4 = 0x12;
  }
  return local_4;
}

