
int __cdecl FUN_0055d670(int param_1,int param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  int iVar3;
  
  *param_3 = 0;
  if ((param_2 < 1) || (3 < param_2)) {
    iVar3 = 0;
  }
  else {
    iVar3 = 1;
  }
  if (iVar3 != 0) {
    puVar1 = DAT_006bb664;
    if (((param_2 != 1) && (puVar1 = DAT_006bb6f4, param_2 != 2)) &&
       (puVar1 = param_3, param_2 == 3)) {
      puVar1 = DAT_006bb69c;
    }
    uVar2 = FUN_0053e190(param_1,(int)puVar1);
    *param_3 = uVar2;
  }
  return iVar3;
}

