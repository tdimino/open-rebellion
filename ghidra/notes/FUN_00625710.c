
int * __cdecl FUN_00625710(undefined4 param_1,undefined4 param_2,int *param_3,char *param_4)

{
  char cVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  char *pcVar5;
  char *pcVar6;
  undefined4 in_stack_ffffffbc;
  undefined2 uVar7;
  uint local_28;
  uint local_24;
  undefined2 local_20;
  short local_1c;
  char local_1a;
  char local_18 [24];
  
  uVar7 = (undefined2)((uint)in_stack_ffffffbc >> 0x10);
  FUN_006257a0(&local_28,&param_1);
  iVar2 = FUN_00627f10(local_28,local_24,CONCAT22(uVar7,local_20),0x11,0,&local_1c);
  param_3[2] = iVar2;
  param_3[1] = (int)local_1c;
  *param_3 = (int)local_1a;
  uVar3 = 0xffffffff;
  pcVar5 = local_18;
  do {
    pcVar6 = pcVar5;
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    pcVar6 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar6;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3;
  pcVar5 = pcVar6 + -uVar3;
  pcVar6 = param_4;
  for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar6 = pcVar6 + 4;
  }
  for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
    *pcVar6 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar6 = pcVar6 + 1;
  }
  param_3[3] = (int)param_4;
  return param_3;
}

