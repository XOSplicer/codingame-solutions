#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
 
 struct node {
    int id;
    struct node **linked;
    int isGateway;
 };
 /*
 struct link {
    struct node *n1, *n2; 
 };*/
 
 void cutLink(int id1, int id2) {
    
    //send cut command
    printf("%d", id1, id2);
 }
 
int main()
{
    int N; // the total number of nodes in the level, including the gateways
    int L; // the number of links
    int E; // the number of exit gateways
    scanf("%d%d%d", &N, &L, &E);
    //printf("info\n");
    struct node nodes[N];
    for(int i=0; i<N; i++) {
        nodes[i].id=i;
        nodes[i].isGateway=0;
        nodes[i].linked=calloc(L, sizeof(*(nodes[i].linked)));
    }
    //printf("nodes\n");
//    struct link links[L];
    for (int i = 0; i < L; i++) {
        int N1; // N1 and N2 defines a link between these nodes
        int N2;
        scanf("%d%d", &N1, &N2);
//        links[i].n1=&nodes[N1];
//        links[i].n2=&nodes[N2];
        struct node **p=nodes[N1].linked;     // Appand n2 linked of n1 
        while(*p) p=p+sizeof(*(nodes[N1].linked));
        *p=&nodes[N2];
        p=nodes[N2].linked;     // Appand n2 linked of n1 
        while(*p) p=p+sizeof(*(nodes[N2].linked));
        *p=&nodes[N1];
    }
    //printf("links\n");
    for (int i = 0; i < E; i++) {
        int EI; // the index of a gateway node
        scanf("%d", &EI);
        nodes[EI].isGateway=1;
    }
    //printf("gateways\n");
    //printf("loaded\n");

    // game loop
    while (1) {
        int SI; // The index of the node on which the Skynet agent is positioned this turn
        scanf("%d", &SI);
        int cut1, cut2;
        struct node **p=nodes[SI].linked;
        //cut direct access to gateway 
        while(*p) {
            if((**p).isGateway) {
                cut1=SI;
                cut2=(**p).id;
                printf("%d %d\n", cut1, cut2);
                break;
            }
            p=p+sizeof(*(nodes[SI].linked));
        }
//        //cut next to agent if no gateway present
//        cut1=SI;
//        cut2=(**(nodes[SI].linked)).id;

        cut1=0;
        cut2=1;

        printf("%d %d\n", cut1, cut2); // Example: 0 1 are the indices of the nodes you wish to sever the link between
    }

    return 0;
}
