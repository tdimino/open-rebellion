
char __fastcall FUN_00509930(void *param_1)

{
  int iVar1;
  int iVar2;
  
  iVar1 = FUN_00507270(param_1,2);
  iVar2 = FUN_00507270(param_1,1);
  if (iVar1 < iVar2) {
    return '\x01';
  }
  iVar1 = FUN_00507270(param_1,2);
  iVar2 = FUN_00507270(param_1,1);
  return (iVar1 <= iVar2) + '\x02';
}

