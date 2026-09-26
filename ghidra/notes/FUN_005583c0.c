
undefined4 * FUN_005583c0(undefined4 param_1,undefined4 param_2,void *param_3)

{
  undefined4 *puVar1;
  undefined4 *unaff_ESI;
  
  if (unaff_ESI != (undefined4 *)0x0) {
    puVar1 = FUN_0058b2c0(unaff_ESI);
    ExceptionList = param_3;
    return puVar1;
  }
  FUN_00619730();
  ExceptionList = param_3;
  return (undefined4 *)0x0;
}

