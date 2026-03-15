
void __fastcall FUN_00438500(void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  
  bVar1 = FUN_006069d0(param_1,0);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    iVar2 = FUN_00401060();
    if (iVar2 == 1) {
      uVar3 = 0x2a31;
    }
    else {
      uVar3 = 0x2a32;
    }
    FUN_00606bd0(param_1,2,2,uVar3);
    FUN_00606b40(param_1,0x3810);
    FUN_00606b40(param_1,0x3811);
    FUN_00606b40(param_1,0x3812);
    FUN_00606b40(param_1,0x3813);
    FUN_00606ab0(param_1);
  }
  return;
}

